mod ast;
//mod async_splitter;
mod asyncness_check;
mod compiler;
mod hir;
mod parser;
mod typing;
use anyhow::{bail, Context, Result};

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let Some(path) = args.get(1) else {
        bail!("usage: milika a.milika > a.mlir");
    };
    let src: String = std::fs::read_to_string(path).context(format!("failed to read {}", path))?;
    let ast = parser::parse(&src)?;
    let hir = typing::run(ast)?;
    //let hir = async_splitter::run(hir)?;
    compiler::run(path, &src, hir)?;
    Ok(())
}
