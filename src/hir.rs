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
    pub is_internal: bool,
    pub name: String,
    pub params: Vec<Param>,
    pub ret_ty: Ty,
}

impl TryFrom<ast::Extern> for Extern {
    type Error = anyhow::Error;
    fn try_from(x: ast::Extern) -> Result<Self> {
        Extern::from_ast(&x)
    }
}

impl Extern {
    pub fn from_ast(x: &ast::Extern) -> Result<Self> {
        Ok(Self {
            is_async: x.is_async,
            is_internal: x.is_internal,
            name: x.name.clone(),
            params: x
                .params
                .iter()
                .map(|x| x.clone().try_into())
                .collect::<Result<_>>()?,
            ret_ty: x.ret_ty.clone().try_into()?,
        })
    }

    pub fn fun_ty(&self) -> FunTy {
        FunTy {
            is_async: self.is_async,
            param_tys: self.params.iter().map(|x| x.ty.clone()).collect::<Vec<_>>(),
            ret_ty: Box::new(self.ret_ty.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub ret_ty: Ty,
    pub body_stmts: Vec<Typed<Expr>>,
}

impl Function {
    pub fn fun_ty(&self, is_async: bool) -> FunTy {
        FunTy {
            is_async,
            param_tys: self.params.iter().map(|x| x.ty.clone()).collect::<Vec<_>>(),
            ret_ty: Box::new(self.ret_ty.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Param {
    pub ty: Ty,
    pub name: String,
}

impl TryFrom<ast::Param> for Param {
    type Error = anyhow::Error;

    fn try_from(x: ast::Param) -> Result<Self> {
        Ok(Self {
            ty: x.ty.try_into()?,
            name: x.name,
        })
    }
}

impl Param {
    pub fn new(ty: Ty, name: impl Into<String>) -> Self {
        Self {
            ty,
            name: name.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    Void,
    Opaque, // Its type is unknown to Milika
    ChiikaEnv,
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
                "void" => Ty::Void,
                "ANY" => Ty::Opaque,
                "ENV" => Ty::ChiikaEnv,
                "FUTURE" => Ty::RustFuture,
                "CONT" => Ty::chiika_cont(),
                "int" => Ty::Int,
                "bool" => Ty::Bool,
                _ => return Err(anyhow!("unknown type: {s}")),
            },
            ast::Ty::Fun(f) => Ty::Fun(f.try_into()?),
        };
        Ok(t)
    }
}

impl Ty {
    pub fn chiika_cont() -> Ty {
        Ty::Fun(FunTy {
            is_async: false,
            param_tys: vec![Ty::ChiikaEnv, Ty::Opaque],
            ret_ty: Box::new(Ty::RustFuture),
        })
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

impl TryFrom<ast::FunTy> for FunTy {
    type Error = anyhow::Error;

    fn try_from(x: ast::FunTy) -> Result<Self> {
        Ok(Self {
            is_async: false,
            param_tys: x
                .param_tys
                .into_iter()
                .map(|x| x.try_into())
                .collect::<Result<_>>()?,
            ret_ty: Box::new((*x.ret_ty).try_into()?),
        })
    }
}

impl FunTy {
    pub fn from_ast_func(f: &ast::Function, is_async: bool) -> Result<Self> {
        Ok(Self {
            is_async,
            param_tys: f
                .params
                .iter()
                .map(|x| x.ty.clone().try_into())
                .collect::<Result<_>>()?,
            ret_ty: Box::new(f.ret_ty.clone().try_into()?),
        })
    }
}

type Typed<T> = (T, Ty);
pub type TypedExpr = Typed<Expr>;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    LVarRef(String),
    ArgRef(usize),
    FuncRef(String),
    OpCall(String, Box<Typed<Expr>>, Box<Typed<Expr>>),
    FunCall(Box<Typed<Expr>>, Vec<Typed<Expr>>),
    If(Box<Typed<Expr>>, Vec<Typed<Expr>>, Option<Vec<Typed<Expr>>>),
    While(Box<Typed<Expr>>, Vec<Typed<Expr>>),
    Alloc(String),
    Assign(String, Box<Typed<Expr>>),
    Return(Box<Typed<Expr>>),
    Para(Vec<Typed<Expr>>),
}

impl Expr {
    pub fn number(n: i64) -> TypedExpr {
        (Expr::Number(n), Ty::Int)
    }

    pub fn arg_ref(idx: usize, ty: Ty) -> TypedExpr {
        (Expr::ArgRef(idx), ty)
    }

    pub fn func_ref(name: impl Into<String>, fun_ty: FunTy) -> TypedExpr {
        (Expr::FuncRef(name.into()), fun_ty.into())
    }

    pub fn op_call(op: impl Into<String>, lhs: TypedExpr, rhs: TypedExpr, ty: Ty) -> TypedExpr {
        (Expr::OpCall(op.into(), Box::new(lhs), Box::new(rhs)), ty)
    }

    pub fn fun_call(func: TypedExpr, args: Vec<TypedExpr>, result_ty: Ty) -> TypedExpr {
        (Expr::FunCall(Box::new(func), args), result_ty)
    }

    pub fn assign(name: impl Into<String>, e: TypedExpr) -> TypedExpr {
        (Expr::Assign(name.into(), Box::new(e)), Ty::Void)
    }

    pub fn return_(e: TypedExpr) -> TypedExpr {
        (Expr::Return(Box::new(e)), Ty::Void)
    }
}
