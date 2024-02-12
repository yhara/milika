use crate::ast::{self, Spanned};
use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0, multispace1},
    multi::many0,
    sequence::delimited,
    IResult,
};
use nom_locate::{self, position};
type Span<'a> = nom_locate::LocatedSpan<&'a str>;

//pub struct Parser<'a> {
//    src: &'a str,
//}
//
//impl<'a> Parser<'a> {
//    pub fn new(src: &str) -> Parser {
//        Parser { src }
//    }
//
//    pub fn parse(&self) -> Result<ast::Program<'a>> {
//        let input = nom_locate::LocatedSpan::new(self.src);
//        let (_, prog) = parse_decls(input)?;
//        Ok(prog)
//    }
//}

pub fn parse(src: &str) -> Result<ast::Program> {
    let input = Span::new(src);
    let (_, prog) = parse_decls(input).map_err(|e| anyhow!("{}", e))?;
    Ok(prog)
}

pub fn parse_decls(s: Span) -> IResult<Span, Vec<ast::Declaration>> {
    Ok((s, Default::default()))
    //many0(parse_decl)(s)
}

fn parse_decl<'a>(s: Span<'a>) -> IResult<Span<'a>, ast::Declaration<'a>> {
    alt((parse_extern, parse_function))(s)
}

fn parse_extern<'a>(s: Span<'a>) -> IResult<Span<'a>, ast::Declaration<'a>> {
    let (s, _) = tag("extern")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, (name, pos)) = parse_ident(s)?;
    let (s, params) = delimited(
        tag("("),
        delimited(multispace0, parse_params, multispace0),
        tag(")"),
    )(s)?;
    let (s, _) = delimited(multispace0, tag("->"), multispace0)(s)?;
    let (s, (ret_ty, _)) = parse_ty(s)?;
    let e = ast::Extern {
        is_async: false,
        name,
        params,
        ret_ty,
    };
    Ok((s, ast::Declaration::Extern((e, pos))))
}

fn parse_function<'a>(_s: Span<'a>) -> IResult<Span<'a>, ast::Declaration<'a>> {
    todo!()
}

fn parse_params<'a>(_s: Span<'a>) -> IResult<Span<'a>, Vec<ast::Param>> {
    todo!()
}

fn parse_varref<'a>(s: Span<'a>) -> IResult<Span<'a>, Spanned<ast::Expr>> {
    let (s, (name, pos)) = parse_ident(s)?;
    Ok((s, (ast::Expr::VarRef(name), pos)))
}

fn parse_ident<'a>(s: Span<'a>) -> IResult<Span<'a>, Spanned<String>> {
    let (s, pos) = position(s)?;
    let (s, name) = alphanumeric1(s)?;
    Ok((s, (name.to_string(), pos)))
}

fn parse_ty<'a>(s: Span<'a>) -> IResult<Span<'a>, Spanned<ast::Ty>> {
    let (s, pos) = position(s)?;
    let (s, name) = alphanumeric1(s)?;
    Ok((s, (ast::Ty::Raw(name.to_string()), pos)))
}

//fn expr_parser<'a>() -> impl Parser<'a, &'a str, ast::Expr> {
//    recursive(|expr| {
//        let atomic = {
//            let number = just('-')
//                .or_not()
//                .chain::<char, _, _>(text::int(10))
//                .collect::<String>()
//                .from_str()
//                .unwrapped()
//                .map(ast::Expr::Number);
//
//            let parenthesized = expr.clone().delimited_by(just('('), just(')'));
//
//            let funcall = (varref_parser().or(parenthesized.clone()))
//                .then_ignore(just('('))
//                .then(expr.clone().padded().separated_by(just(',')))
//                .then_ignore(just(')'))
//                .map(|(func_expr, args)| ast::Expr::FunCall(Box::new(func_expr), args));
//
//            funcall.or(parenthesized).or(varref_parser()).or(number)
//        };
//        let bin_op = just("==")
//            .or(just("!="))
//            .or(just("<="))
//            .or(just("<"))
//            .or(just(">="))
//            .or(just(">"))
//            .or(just("+"))
//            .or(just("-"));
//        let sum = atomic
//            .clone()
//            .then(bin_op.padded().then(atomic.clone()).repeated())
//            .foldl(|lhs, (op, rhs)| {
//                ast::Expr::OpCall(op.to_string(), Box::new(lhs), Box::new(rhs))
//            });
//
//        let alloc = just("alloc")
//            .padded()
//            .ignore_then(ident_parser())
//            .map(ast::Expr::Alloc);
//
//        let retn = just("return")
//            .padded()
//            .ignore_then(expr.clone())
//            .map(|e| ast::Expr::Return(Box::new(e)));
//
//        //        let para = just("para")
//        //            .padded()
//        //            .ignore_then(
//
//        let assign = ident_parser()
//            .padded()
//            .then_ignore(just('=').padded())
//            .then(expr.clone())
//            .map(|(name, rhs)| ast::Expr::Assign(name, Box::new(rhs)));
//
//        alloc.or(retn).or(assign).or(sum).or(atomic)
//    })
//}
//
//fn stmts_parser<'a>() -> impl Parser<'a, &'a str, Vec<ast::Expr>> {
//    let comment = just('#').then(take_until(just('\n'))).padded();
//    expr_parser()
//        .padded()
//        .then_ignore(just(';'))
//        .padded_by(comment.repeated())
//        .padded()
//        .repeated()
//}
//
//fn param_parser<'a>() -> impl Parser<'a, &'a str, ast::Param> {
//    ty_parser()
//        .padded()
//        .then(ident_parser())
//        .map(|(ty, name)| ast::Param { ty, name })
//}
//
//fn params_parser<'a>() -> impl Parser<'a, &'a str, Vec<ast::Param>> {
//    param_parser().padded().separated_by(just(','))
//}
//
//fn func_parser<'a>() -> impl Parser<'a, &'a str, Spanned<ast::Function>> {
//    just("fun")
//        .ignore_then(ident_parser().padded())
//        .then(params_parser().delimited_by(just('('), just(')')))
//        .then_ignore(just("->").padded())
//        .then(ty_parser().padded())
//        .then(stmts_parser().delimited_by(just('{'), just('}')))
//        .map_with_span(|(((name, params), ret_ty), body_stmts), span| {
//            let f = ast::Function {
//                name,
//                params,
//                ret_ty,
//                body_stmts,
//            };
//            (f, span)
//        })
//}
//
//fn extern_parser<'a>() -> impl Parser<'a, &'a str, Spanned<ast::Extern>> {
//    just("extern_async")
//        .or(just("extern"))
//        .then(ident_parser().padded())
//        .then(params_parser().delimited_by(just('('), just(')')))
//        .then_ignore(just("->").padded())
//        .then(ty_parser().padded())
//        .then_ignore(just(';').padded())
//        .map_with_span(|(((is_async, name), params), ret_ty), span| {
//            let e = ast::Extern {
//                is_async: is_async == "extern_async",
//                name,
//                params,
//                ret_ty,
//            };
//            (e, span)
//        })
//}
//
//fn decl_parser<'a>() -> impl Parser<'a, &'a str, ast::Declaration> {
//    func_parser()
//        .map(ast::Declaration::Function)
//        .or(extern_parser().map(ast::Declaration::Extern))
//}
//
//pub fn parser<'a>() -> impl Parser<'a, &'a str, ast::Program> {
//    let comment = just('#').then(take_until(just('\n'))).padded();
//    decl_parser()
//        .padded_by(comment.repeated())
//        .padded()
//        .repeated()
//        .then_ignore(end())
//}
