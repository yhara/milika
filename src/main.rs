mod ast;
mod asyncness_check;
mod compiler;
mod parser;
use anyhow::{anyhow, bail, Context, Result};
//use ariadne::{Label, Report, ReportKind, Source};
use nom::bytes::complete::{tag, take_until};
use nom_locate::{position, LocatedSpan};
type Span<'a> = LocatedSpan<&'a str>;
use nom::IResult;
fn parse_foobar(s: Span) -> IResult<Span, Vec<ast::Declaration>> {
    let (s, _) = take_until("foo")(s)?;
    let (s, pos) = position(s)?;
    let (s, foo) = tag("foo")(s)?;
    let (s, bar) = tag("bar")(s)?;

    Ok((
        s,
        vec![], //        Token {
                //            position: pos,
                //            _foo: foo.fragment(),
                //            _bar: bar.fragment(),
                //        },
    ))
}

//fn render_parse_error(src: &str, span: std::ops::Range<usize>, msg: String) -> String {
//    let mut rendered = vec![];
//    Report::build(ReportKind::Error, "", span.start)
//        .with_message(msg.clone())
//        .with_label(Label::new(("", span)).with_message(msg))
//        .finish()
//        .write(("", Source::from(src)), &mut rendered)
//        .unwrap();
//    String::from_utf8_lossy(&rendered).to_string()
//}

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let Some(path) = args.get(1) else {
        bail!("usage: milika a.milika > a.mlir");
    };
    let src: String = std::fs::read_to_string(path).context(format!("failed to read {}", path))?;
    //let p = parser::Parser::new(&src);
    //let ast = p.parse()?;
    //let input = Span::new(&src);
    //let output = parser::parse_decls(input).context("??")?;
    //let output = parser::parse_decls(input).map_err(|e| anyhow!("..."))?;
    let ast = parser::parse(&src)?;
    //    compiler::run(path, &src, ast)?;
    dbg!(&"ok");
    Ok(())
}
