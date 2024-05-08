mod ast;
mod compiler;
mod hir;
mod hir_lowering;
mod parser;
mod prelude;
mod verifier;
use anyhow::{bail, Context, Result};
use ariadne::{Label, Report, ReportKind, Source};

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let Some(path) = args.get(1) else {
        bail!("usage: milika a.milika > a.mlir");
    };
    let src = std::fs::read_to_string(path).context(format!("failed to read {}", path))?;
    let mut bhir = compile(&src, &path, false)?;

    let prelude_txt = prelude::prelude_funcs(main_is_async(&bhir)?);
    let mut prelude_hir = compile(&prelude_txt, "src/prelude.rs", true)?;
    for e in prelude_hir.externs {
        if !e.is_internal {
            bhir.externs.push(e);
        }
    }
    bhir.funcs.append(&mut prelude_hir.funcs);

    println!("-- verifier input --\n{bhir}\n");
    verifier::run(&bhir)?;

    println!("{bhir}");
    compiler::run(path, &src, bhir)?;
    Ok(())
}

fn compile(src: &str, path: &str, is_prelude: bool) -> Result<hir::blocked::Program> {
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
    let mut hir = hir::untyped::create(&ast)?;
    hir::typing::run(&mut hir)?;
    debug(format!("-- typing output --\n{hir}\n"), !is_prelude);
    let hir = hir_lowering::lower_async_if::run(hir)?;
    debug(format!("-- lower_async_if output --\n{hir}\n"), !is_prelude);
    let hir = hir::asyncness_check::run(hir);
    debug(
        format!("-- asyncness_check output --\n{hir}\n"),
        !is_prelude,
    );
    let hir = hir_lowering::async_splitter::run(hir)?;
    debug(format!("-- async_splitter output --\n{hir}\n"), !is_prelude);
    let bhir = hir_lowering::lower_if::run(hir);
    Ok(bhir)
}

fn debug(s: String, print: bool) {
    if print {
        println!("{}", s);
    }
}

fn main_is_async(hir: &hir::blocked::Program) -> Result<bool> {
    let Some(main) = hir.funcs.iter().find(|x| x.name == "chiika_main") else {
        bail!("chiika_main not found");
    };
    // When chiika_main calls async function, it is lowered to take a continuation.
    Ok(main.params.len() > 0)
}
