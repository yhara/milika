use crate::hir;

pub type Block = Vec<hir::Typed<hir::Expr>>;

#[derive(Debug, Clone)]
pub struct Program {
    pub externs: Vec<hir::Extern>,
    pub funcs: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub is_async: Option<bool>, // None means "unknown" or "N/A" depending on the phase
    pub name: String,
    pub params: Vec<hir::Param>,
    pub ret_ty: hir::Ty,
    pub body_blocks: Vec<Block>,
}
