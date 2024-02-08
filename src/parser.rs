use crate::ast;
use chumsky::prelude::*;

fn ty_parser() -> impl Parser<char, ast::Ty, Error = Simple<char>> {
    recursive(|_ty| {
        //let params = ty
        //    .clone()
        //    .padded()
        //    .separated_by(just(','))
        //    .delimited_by(just('('), just(')'));
        //let sig = params
        //    .then_ignore(just("->").padded())
        //    .then(ty.clone())
        //    .delimited_by(just('('), just(')'));
        //let fn_ty = just("$FN")
        //    .ignore_then(sig)
        //    .map(|(param_tys, ret_ty)| ast::Ty::fun(param_tys, ret_ty));

        let raw_ty = ident_parser().map(|name| ast::Ty::Raw(name));

        raw_ty
    })
}

fn ident_parser() -> impl Parser<char, String, Error = Simple<char>> {
    text::ident()
}

fn varref_parser() -> impl Parser<char, ast::Expr, Error = Simple<char>> {
    ident_parser().map(ast::Expr::VarRef)
}

fn atomic_parser(
    expr_parser: impl Parser<char, ast::Expr, Error = Simple<char>> + Clone,
) -> impl Parser<char, ast::Expr, Error = Simple<char>> {
    let number = just('-')
        .or_not()
        .chain::<char, _, _>(text::int(10))
        .collect::<String>()
        .from_str()
        .unwrapped()
        .map(ast::Expr::Number);

    let parenthesized = expr_parser.clone().delimited_by(just('('), just(')'));

    let funcall = (varref_parser().or(parenthesized.clone()))
        .then_ignore(just('('))
        .then(expr_parser.clone().padded().separated_by(just(',')))
        .then_ignore(just(')'))
        .map(|(func_expr, args)| ast::Expr::FunCall(Box::new(func_expr), args));

    funcall.or(parenthesized).or(varref_parser()).or(number)
}

fn expr_parser() -> impl Parser<char, ast::Expr, Error = Simple<char>> {
    recursive(|expr| {
        let bin_op = just("==")
            .or(just("!="))
            .or(just("<="))
            .or(just("<"))
            .or(just(">="))
            .or(just(">"))
            .or(just("+"))
            .or(just("-"));
        let sum = atomic_parser(expr.clone())
            .then(bin_op.padded().then(atomic_parser(expr.clone())).repeated())
            .foldl(|lhs, (op, rhs)| {
                ast::Expr::OpCall(op.to_string(), Box::new(lhs), Box::new(rhs))
            });

        let alloc = just("alloc")
            .padded()
            .ignore_then(ident_parser())
            .map(ast::Expr::Alloc);

        let assign = ident_parser()
            .padded()
            .then_ignore(just('=').padded())
            .then(expr.clone())
            .map(|(name, rhs)| ast::Expr::Assign(name, Box::new(rhs)));

        alloc.or(assign).or(sum).or(atomic_parser(expr))
    })
}

fn stmts_parser() -> impl Parser<char, Vec<ast::Expr>, Error = Simple<char>> {
    expr_parser()
        .padded()
        .separated_by(just(';'))
        .allow_trailing()
}

fn param_parser() -> impl Parser<char, ast::Param, Error = Simple<char>> {
    ty_parser()
        .padded()
        .then(ident_parser())
        .map(|(ty, name)| ast::Param { ty, name })
}

fn params_parser() -> impl Parser<char, Vec<ast::Param>, Error = Simple<char>> {
    param_parser().padded().separated_by(just(','))
}

fn func_parser() -> impl Parser<char, ast::Function, Error = Simple<char>> {
    just("fun")
        .ignore_then(ident_parser().padded())
        .then(params_parser().delimited_by(just('('), just(')')))
        .then_ignore(just("->").padded())
        .then(ty_parser().padded())
        .then(stmts_parser().delimited_by(just('{'), just('}')))
        .map(|(((name, params), ret_ty), body_stmts)| ast::Function {
            name,
            params,
            ret_ty,
            body_stmts,
        })
}

fn extern_parser() -> impl Parser<char, ast::Extern, Error = Simple<char>> {
    just("extern_async")
        .or(just("extern"))
        .then(ident_parser().padded())
        .then(params_parser().delimited_by(just('('), just(')')))
        .then_ignore(just("->").padded())
        .then(ty_parser().padded())
        .then_ignore(just(';').padded())
        .map(|(((is_async, name), params), ret_ty)| ast::Extern {
            is_async: is_async == "extern_async",
            name,
            params,
            ret_ty,
        })
}

fn decl_parser() -> impl Parser<char, ast::Declaration, Error = Simple<char>> {
    func_parser()
        .map(ast::Declaration::Function)
        .or(extern_parser().map(ast::Declaration::Extern))
}

pub fn parser() -> impl Parser<char, ast::Program, Error = Simple<char>> {
    decl_parser().padded().repeated().then_ignore(end())
}
