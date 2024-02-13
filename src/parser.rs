use crate::ast::{self, Spanned};
use anyhow::{anyhow, Result};
//use ariadne::{Label, Report, ReportKind, Source};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{alphanumeric1, multispace0, multispace1, one_of},
    combinator::eof,
    multi::{many0, separated_list0},
    number,
    sequence::{delimited, preceded, terminated},
    IResult,
};
use nom_locate::{self, position};
type Span<'a> = nom_locate::LocatedSpan<&'a str>;
type E<'a> = nom::error::VerboseError<Span<'a>>;

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
    alt((parse_extern, parse_function))(s)
}

fn parse_extern<'a>(s: Span<'a>) -> IResult<Span<'a>, ast::Declaration<'a>, E> {
    let (s, _) = tag("extern")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, (name, pos)) = parse_ident(s)?;
    let (s, params) = parse_param_list(s)?;
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

fn parse_param_list<'a>(s: Span<'a>) -> IResult<Span<'a>, Vec<ast::Param>, E> {
    delimited(
        tag("("),
        delimited(multispace0, parse_params, multispace0),
        tag(")"),
    )(s)
}

fn parse_params<'a>(s: Span<'a>) -> IResult<Span<'a>, Vec<ast::Param>, E> {
    many0(delimited(multispace0, parse_param, multispace0))(s)
}

fn parse_param<'a>(s: Span<'a>) -> IResult<Span<'a>, ast::Param, E> {
    let (s, (ty, _)) = parse_ty(s)?;
    let (s, _) = multispace1(s)?;
    let (s, (name, _)) = parse_ident(s)?;
    Ok((s, ast::Param { ty, name }))
}

fn parse_function<'a>(s: Span<'a>) -> IResult<Span<'a>, ast::Declaration<'a>, E> {
    let (s, _) = tag("fun")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, (name, pos)) = parse_ident(s)?;
    let (s, params) = parse_param_list(s)?;
    let (s, _) = delimited(multispace0, tag("->"), multispace0)(s)?;
    let (s, (ret_ty, _)) = parse_ty(s)?;
    let (s, body_stmts) = preceded(multispace0, parse_block)(s)?;
    let e = ast::Function {
        name,
        params,
        ret_ty,
        body_stmts,
    };
    Ok((s, ast::Declaration::Function((e, pos))))
}

fn parse_block<'a>(s: Span<'a>) -> IResult<Span<'a>, Vec<ast::Expr>, E> {
    delimited(
        tag("{"),
        many0(delimited(parse_comments, parse_stmt, parse_comments)),
        tag("}"),
    )(s)
}

/// An expr terminated with ';'
fn parse_stmt<'a>(s: Span<'a>) -> IResult<Span<'a>, ast::Expr, E> {
    terminated(
        alt((parse_return, parse_expr)),
        terminated(multispace0, tag(";")),
    )(s)
}

fn parse_return<'a>(s: Span<'a>) -> IResult<Span<'a>, ast::Expr, E> {
    let (s, _) = tag("return")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, expr) = parse_expr(s)?;
    Ok((s, ast::Expr::Return(Box::new(expr))))
}

fn parse_expr<'a>(s: Span<'a>) -> IResult<Span<'a>, ast::Expr, E> {
    alt((
        parse_additive,
        alt((
            parse_para,
            alt((parse_funcall, alt((parse_number, parse_varref)))),
        )),
    ))(s)
}

fn parse_additive<'a>(s: Span<'a>) -> IResult<Span<'a>, ast::Expr, E> {
    let (s, left) = parse_multiplicative(s)?;
    let (s, op) = delimited(multispace1, one_of("+-"), multispace1)(s)?;
    let (s, right) = parse_multiplicative(s)?;
    Ok((
        s,
        ast::Expr::OpCall(op.to_string(), Box::new(left), Box::new(right)),
    ))
}

fn parse_multiplicative<'a>(s: Span<'a>) -> IResult<Span<'a>, ast::Expr, E> {
    let (s, left) = parse_atomic(s)?;
    let (s, op) = delimited(multispace1, one_of("*/"), multispace1)(s)?;
    let (s, right) = parse_atomic(s)?;
    Ok((
        s,
        ast::Expr::OpCall(op.to_string(), Box::new(left), Box::new(right)),
    ))
}

fn parse_atomic<'a>(s: Span<'a>) -> IResult<Span<'a>, ast::Expr, E> {
    alt((
        parse_para,
        alt((parse_funcall, alt((parse_number, parse_varref)))),
    ))(s)
}

fn parse_para<'a>(s: Span<'a>) -> IResult<Span<'a>, ast::Expr, E> {
    let (s, _) = tag("para")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, exprs) = parse_block(s)?;
    Ok((s, ast::Expr::Para(exprs)))
}

fn parse_funcall<'a>(s: Span<'a>) -> IResult<Span<'a>, ast::Expr, E> {
    let (s, f) = parse_varref(s)?;
    let (s, args) = parse_arg_list(s)?;
    Ok((s, ast::Expr::FunCall(Box::new(f), args)))
}

fn parse_arg_list<'a>(s: Span<'a>) -> IResult<Span<'a>, Vec<ast::Expr>, E> {
    delimited(
        tag("("),
        delimited(multispace0, parse_args, multispace0),
        tag(")"),
    )(s)
}

fn parse_args<'a>(s: Span<'a>) -> IResult<Span<'a>, Vec<ast::Expr>, E> {
    separated_list0(delimited(multispace0, tag(","), multispace0), parse_expr)(s)
}

fn parse_number<'a>(s: Span<'a>) -> IResult<Span<'a>, ast::Expr, E> {
    // TODO: Just parse integer
    let (s, n) = number::complete::double(s)?;
    Ok((s, ast::Expr::Number(n.floor() as i64)))
}

fn parse_varref<'a>(s: Span<'a>) -> IResult<Span<'a>, ast::Expr, E> {
    let (s, (name, _pos)) = parse_ident(s)?;
    Ok((s, ast::Expr::VarRef(name)))
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
