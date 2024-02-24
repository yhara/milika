use crate::ast;

#[derive(Debug, Clone)]
pub struct Program {
    pub externs: Vec<Extern>,
    pub funcs: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct Extern {
    pub is_async: bool,
    pub name: String,
    pub params: Vec<Param>,
    pub ret_ty: Ty,
}

impl From<ast::Extern> for Extern {
    fn from(x: ast::Extern) -> Self {
        Self {
            is_async: x.is_async,
            name: x.name,
            params: x.params.into_iter().map(|x| x.into()).collect(),
            ret_ty: x.ret_ty.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub ret_ty: Ty,
    pub body_stmts: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub ty: Ty,
    pub name: String,
}

impl From<ast::Param> for Param {
    fn from(x: ast::Param) -> Self {
        Self {
            ty: x.ty.into(),
            name: x.name,
        }
    }
}

impl Param {
    pub fn new(ty: Ty, name: &str) -> Param {
        Param {
            ty,
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Ty {
    Opaque, // Its type is unknown to Milika
    ChiikaEnv,
    ChiikaCont,
    RustFuture,
    Raw(String),
    Fun(FunTy),
}

impl From<ast::Ty> for Ty {
    fn from(x: ast::Ty) -> Self {
        match x {
            ast::Ty::Raw(s) => match &s[..] {
                "ENV" => Ty::ChiikaEnv,
                "CONT" => Ty::ChiikaCont,
                "FUTURE" => Ty::RustFuture,
                _ => Ty::Raw(s),
            },
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunTy {
    pub is_async: bool,
    pub param_tys: Vec<Ty>,
    pub ret_ty: Box<Ty>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    VarRef(String),
    OpCall(String, Box<Expr>, Box<Expr>),
    FunCall(Box<Expr>, Vec<Expr>),
    If(Box<Expr>, Vec<Expr>, Option<Vec<Expr>>),
    While(Box<Expr>, Vec<Expr>),
    Cast(Box<Expr>, Ty),
    Alloc(String),
    Assign(String, Box<Expr>),
    Return(Box<Expr>),
    Para(Vec<Expr>),
}

impl Expr {
    pub fn var_ref(name: impl Into<String>) -> Expr {
        Expr::VarRef(name.into())
    }
}
