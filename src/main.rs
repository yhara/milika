mod ast;
mod async_splitter;
mod asyncness_check;
mod compiler;
mod hir;
mod parser;
mod prelude;
mod typing;
mod verifier;
use anyhow::{bail, Context, Result};
use ariadne::{Label, Report, ReportKind, Source};

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let Some(path) = args.get(1) else {
        bail!("usage: milika a.milika > a.mlir");
    };
    let src = std::fs::read_to_string(path).context(format!("failed to read {}", path))?;
    let mut hir = compile(&src, &path)?;

    let prelude_txt = prelude::prelude_funcs(main_is_async(&hir)?);
    let mut prelude_hir = compile(&prelude_txt, "src/prelude.rs")?;
    for e in prelude_hir.externs {
        if !e.is_internal {
            hir.externs.push(e);
        }
    }
    hir.funcs.append(&mut prelude_hir.funcs);

    verifier::run(&hir)?;

    println!("{hir}");
    compiler::run(path, &src, hir)?;
    Ok(())
}

fn compile(src: &str, path: &str) -> Result<hir::Program> {
    let ast = match parser::parse(src) {
        Ok(ast) => ast,
        Err(e) => {
            dbg!(&e);
            let span = e.location.offset..e.location.offset;
            Report::build(ReportKind::Error, path, e.location.offset)
                .with_label(Label::new((path, span)).with_message("here"))
                .finish()
                .print((path, Source::from(src)))
                .unwrap();
            bail!("parse error");
        }
    };
    let hir = typing::run(ast)?;
    let hir = async_splitter::run(hir)?;
    Ok(hir)
}

fn main_is_async(hir: &hir::Program) -> Result<bool> {
    let Some(main) = hir.funcs.iter().find(|x| x.name == "chiika_main") else {
        bail!("chiika_main not found");
    };
    // When chiika_main calls async function, it is lowered to take a continuation.
    Ok(main.params.len() > 0)
}
