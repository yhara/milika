use crate::ast;
use anyhow::{anyhow, Result};

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
        Extern::from_ast(&x)
    }
}

impl Extern {
    pub fn from_ast(x: &ast::Extern) -> Self {
        Self {
            is_async: x.is_async,
            name: x.name.clone(),
            params: x.params.into_iter().map(|x| x.into()).collect(),
            ret_ty: x.ret_ty.into(),
        }
    }

    pub fn fun_ty(&self) -> FunTy {
        FunTy {
            is_async: self.is_async,
            param_tys: self.params.iter().map(|x| x.ty.clone()).collect::<Vec<_>>(),
            ret_ty: Box::new(self.ret_ty.clone()),
        }
    }

    //pub fn into_empty_func(self) -> Function {
    //    Function {
    //        name: self.name,
    //        params: self.params,
    //        ret_ty: self.ret_ty,
    //        body_stmts: Default::default(),
    //    }
    //}
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub ret_ty: Ty,
    pub body_stmts: Vec<Typed<Expr>>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    Void,
    Opaque, // Its type is unknown to Milika
    ChiikaEnv,
    ChiikaCont,
    RustFuture,
    Int,
    Bool,
    Fun(FunTy),
}

impl TryFrom<ast::Ty> for Ty {
    type Error = anyhow::Error;

    fn try_from(x: ast::Ty) -> Result<Self> {
        let t = match x {
            ast::Ty::Raw(s) => match &s[..] {
                "ENV" => Ty::ChiikaEnv,
                "CONT" => Ty::ChiikaCont,
                "FUTURE" => Ty::RustFuture,
                "int" => Ty::Int,
                "bool" => Ty::Bool,
                _ => return Err(anyhow!("unknown type: {s}")),
            },
            _ => todo!(),
        };
        Ok(t)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunTy {
    pub is_async: bool,
    pub param_tys: Vec<Ty>,
    pub ret_ty: Box<Ty>,
}

impl From<FunTy> for Ty {
    fn from(x: FunTy) -> Self {
        Ty::Fun(x)
    }
}

impl FunTy {
    pub fn from_ast_func(f: &ast::Function, is_async: bool) -> Self {
        Self {
            is_async,
            param_tys: f.params.iter().map(|x| x.clone().into()).collect(),
            ret_ty: f.ret_ty.into(),
        }
    }
}

type Typed<T> = (T, Ty);

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    VarRef(String),
    OpCall(String, Box<Typed<Expr>>, Box<Typed<Expr>>),
    FunCall(Box<Typed<Expr>>, Vec<Typed<Expr>>),
    If(Box<Typed<Expr>>, Vec<Typed<Expr>>, Option<Vec<Typed<Expr>>>),
    While(Box<Typed<Expr>>, Vec<Typed<Expr>>),
    Cast(Box<Typed<Expr>>, Ty),
    Alloc(String),
    Assign(String, Box<Typed<Expr>>),
    Return(Box<Typed<Expr>>),
    Para(Vec<Typed<Expr>>),
}

impl Expr {
    pub fn var_ref(name: impl Into<String>) -> Expr {
        Expr::VarRef(name.into())
    }
}
