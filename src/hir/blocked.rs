use crate::hir;
use std::fmt;

pub type Block = Vec<hir::Typed<hir::Expr>>;

#[derive(Debug, Clone)]
pub struct Program {
    pub externs: Vec<hir::Extern>,
    pub funcs: Vec<Function>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for e in &self.externs {
            write!(f, "{}", e)?;
        }
        for func in &self.funcs {
            write!(f, "{}", func)?;
        }
        write!(f, "")
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub is_async: Option<bool>, // None means "unknown" or "N/A" depending on the phase
    pub name: String,
    pub params: Vec<hir::Param>,
    pub ret_ty: hir::Ty,
    pub body_blocks: Vec<Block>,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let para = self
            .params
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "fun {}({}) -> {} {{\n", self.name, para, self.ret_ty)?;
        for block in &self.body_blocks {
            write!(f, "^bb()\n")?;
            for expr in block {
                write!(f, "  {};  #-> {}\n", &expr.0, &expr.1)?;
            }
        }
        write!(f, "}}\n")
    }
}
