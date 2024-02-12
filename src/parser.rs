use crate::ast::{self, Spanned};
use anyhow::{anyhow, Result};
//use ariadne::{Label, Report, ReportKind, Source};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{alphanumeric1, multispace0, multispace1},
    combinator::eof,
    multi::many0,
    sequence::{delimited, preceded},
    IResult,
};
use nom_locate::{self, position};
type Span<'a> = nom_locate::LocatedSpan<&'a str>;
type E<'a> = nom::error::VerboseError<Span<'a>>;

//impl<'a> core::ops::Deref for Span<'a> {
//    type Target = &'a str;
//    fn deref(&self) -> &Self::Target {
//        &self.fragment
//    }
//}

//fn render_parse_error(src: &str, e: E) -> String {
//    let mut rendered = vec![];
//    Report::build(ReportKind::Error, "", span.start)
//        .with_message(msg.clone())
//        .with_label(Label::new(("", span)).with_message(msg))
//        .finish()
//        .write(("", Source::from(src)), &mut rendered)
//        .unwrap();
//    String::from_utf8_lossy(&rendered).to_string()
//}

pub fn parse(src: &str) -> Result<ast::Program> {
    let input = Span::new(src);
    match parse_decls(input) {
        Ok((_, prog)) => Ok(prog),
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
            // https://github.com/fflorent/nom_locate/issues/36#issuecomment-1013469728
            let errors = e
                .errors
                .into_iter()
                .map(|(input, error)| (*input.fragment(), error))
                .collect();
            let s = nom::error::convert_error(src, nom::error::VerboseError { errors });
            Err(anyhow!("{}", s))
        }
        _ => unreachable!(),
    }
}

fn parse_decls(s: Span) -> IResult<Span, Vec<ast::Declaration>, E> {
    let (s, decls) = many0(delimited(parse_comments, parse_decl, parse_comments))(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = eof(s)?;
    Ok((s, decls))
    //Ok((s, Default::default()))
}

fn parse_comments(s: Span) -> IResult<Span, (), E> {
    let (s, _) = multispace0(s)?;
    let (s, _) = many0(delimited(multispace0, parse_comment, multispace0))(s)?;
    Ok((s, ()))
}

fn parse_comment(s: Span) -> IResult<Span, (), E> {
    let (s, _) = tag("#")(s)?;
    let (s, _) = take_till(|c| c == '\n')(s)?;
    Ok((s, ()))
}

fn parse_decl<'a>(s: Span<'a>) -> IResult<Span<'a>, ast::Declaration<'a>, E> {
    parse_extern(s)
    //alt((parse_extern, parse_function))(s)
}

fn parse_extern<'a>(s: Span<'a>) -> IResult<Span<'a>, ast::Declaration<'a>, E> {
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
    let (s, _) = preceded(multispace0, tag(";"))(s)?;
    let e = ast::Extern {
        is_async: false,
        name,
        params,
        ret_ty,
    };
    Ok((s, ast::Declaration::Extern((e, pos))))
}

//fn parse_function<'a>(s: Span<'a>) -> IResult<Span<'a>, ast::Declaration<'a>> {
//    tag("fun")(s)
//}

fn parse_params<'a>(s: Span<'a>) -> IResult<Span<'a>, Vec<ast::Param>, E> {
    many0(delimited(multispace0, parse_param, multispace0))(s)
}

fn parse_param<'a>(s: Span<'a>) -> IResult<Span<'a>, ast::Param, E> {
    let (s, (ty, _)) = parse_ty(s)?;
    let (s, _) = multispace1(s)?;
    let (s, (name, _)) = parse_ident(s)?;
    Ok((s, ast::Param { ty, name }))
}

fn parse_varref<'a>(s: Span<'a>) -> IResult<Span<'a>, Spanned<ast::Expr>, E> {
    let (s, (name, pos)) = parse_ident(s)?;
    Ok((s, (ast::Expr::VarRef(name), pos)))
}

fn parse_ident<'a>(s: Span<'a>) -> IResult<Span<'a>, Spanned<String>, E> {
    let (s, pos) = position(s)?;
    let (s, name) = alphanumeric1(s)?;
    Ok((s, (name.to_string(), pos)))
}

fn parse_ty<'a>(s: Span<'a>) -> IResult<Span<'a>, Spanned<ast::Ty>, E> {
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
