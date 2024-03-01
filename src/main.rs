mod ast;
//mod async_splitter;
mod asyncness_check;
mod compiler;
mod hir;
mod parser;
mod prelude;
mod typing;
use anyhow::{bail, Context, Result};

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let Some(path) = args.get(1) else {
        bail!("usage: milika a.milika > a.mlir");
    };
    let src = std::fs::read_to_string(path).context(format!("failed to read {}", path))?;
    let mut hir = compile(&src)?;

    let main_is_async = hir
        .funcs
        .iter()
        .find(|x| x.name == "chiika_main")
        .map(|x| x.is_async.expect("[BUG] chiika_main's asyncness not known"))
        .expect("chiika_main not found");

    let prelude_txt = prelude::prelude_funcs(main_is_async);
    let mut prelude_hir = compile(&prelude_txt)?;
    for e in prelude_hir.externs {
        if !e.is_internal {
            hir.externs.push(e);
        }
    }
    hir.funcs.append(&mut prelude_hir.funcs);

    compiler::run(path, &src, hir)?;
    Ok(())
}

fn compile(src: &str) -> Result<hir::Program> {
    let ast = parser::parse(src)?;
    let hir = typing::run(ast)?;
    let hir = async_splitter::run(hir)?;
    Ok(hir)
}
