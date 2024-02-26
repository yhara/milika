mod ast;
mod typing;
//mod ast2hir;
mod asyncness_check;
//mod compiler;
mod hir;
mod parser;
use anyhow::{bail, Context, Result};

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let Some(path) = args.get(1) else {
        bail!("usage: milika a.milika > a.mlir");
    };
    let src: String = std::fs::read_to_string(path).context(format!("failed to read {}", path))?;
    let ast = parser::parse(&src)?;
    dbg!(&typing::run(ast)?);
    //let prog = ast2hir::run(ast)?;
    //dbg!(&prog);
    //compiler::run(path, &src, ast)?;
    Ok(())
}
