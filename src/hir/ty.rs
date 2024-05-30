use crate::ast;
use crate::hir::Asyncness;
use anyhow::{anyhow, Result};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    Unknown, // Used before typecheck
    Null,    // A unit type. Represented by `i64 0`
    Void,    // eg. the type of `return` or assignment. There is no value of this type.
    Any,     // Corresponds to `ptr` in llvm
    ChiikaEnv,
    RustFuture,
    Int,
    Bool,
    Fun(FunTy),
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Fun(fun_ty) => write!(f, "{}", fun_ty),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl TryFrom<ast::Ty> for Ty {
    type Error = anyhow::Error;

    fn try_from(x: ast::Ty) -> Result<Self> {
        let t = match x {
            ast::Ty::Raw(s) => match &s[..] {
                "Null" => Ty::Null,
                "Int" => Ty::Int,
                "Bool" => Ty::Bool,
                // Internally used types (in src/prelude.rs)
                "ANY" => Ty::Any,
                "ENV" => Ty::ChiikaEnv,
                "FUTURE" => Ty::RustFuture,
                "CONT" => Ty::chiika_cont(),
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
            asyncness: Asyncness::Lowered,
            param_tys: vec![Ty::ChiikaEnv, Ty::Any],
            ret_ty: Box::new(Ty::RustFuture),
        })
    }

    pub fn as_fun_ty(&self) -> &FunTy {
        match self {
            Ty::Fun(f) => f,
            _ => panic!("[BUG] not a function type: {:?}", self),
        }
    }

    pub fn into_fun_ty(self) -> FunTy {
        match self {
            Ty::Fun(f) => f,
            _ => panic!("[BUG] not a function type: {:?}", self),
        }
    }

    pub fn is_async_fun(&self) -> bool {
        match self {
            Ty::Fun(f) => f.asyncness.is_async(),
            _ => panic!("[BUG] not a function type: {:?}", self),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct FunTy {
    pub asyncness: Asyncness,
    pub param_tys: Vec<Ty>,
    pub ret_ty: Box<Ty>,
}

impl fmt::Display for FunTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let para = self
            .param_tys
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(",");
        write!(f, "({})->{}", para, &self.ret_ty)
    }
}

impl fmt::Debug for FunTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl From<FunTy> for Ty {
    fn from(x: FunTy) -> Self {
        Ty::Fun(x)
    }
}

impl From<Ty> for FunTy {
    fn from(x: Ty) -> Self {
        match x {
            Ty::Fun(f) => f,
            _ => panic!("[BUG] not a function type: {:?}", x),
        }
    }
}

impl TryFrom<ast::FunTy> for FunTy {
    type Error = anyhow::Error;

    fn try_from(x: ast::FunTy) -> Result<Self> {
        Ok(Self {
            asyncness: Asyncness::Unknown,
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
    /// Returns true if the two function types are the same except for asyncness.
    pub fn same(&self, other: &Self) -> bool {
        self.param_tys == other.param_tys && self.ret_ty == other.ret_ty
    }
}
