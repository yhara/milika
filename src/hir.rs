pub mod typing;
pub mod untyped;
use crate::ast;
use anyhow::{anyhow, Result};
use std::fmt;

pub type Program = Program_<Ty>;

#[derive(Debug, Clone)]
pub struct Program_<TY> {
    pub externs: Vec<Extern>,
    pub funcs: Vec<Function_<TY>>,
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

impl fmt::Display for Extern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let asyn = if self.is_async { "(async)" } else { "" };
        let inte = if self.is_internal { "(internal)" } else { "" };
        let para = self
            .params
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(
            f,
            "extern{}{} {}({}) -> {};\n",
            asyn, inte, self.name, para, self.ret_ty
        )
    }
}

pub type Function = Function_<Ty>;

#[derive(Debug, Clone)]
pub struct Function_<TY> {
    pub name: String,
    pub params: Vec<Param>,
    pub ret_ty: Ty,
    pub body_stmts: Vec<Typed<Expr_<TY>, TY>>,
}

impl<TY: fmt::Display> fmt::Display for Function_<TY> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let para = self
            .params
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "fun {}({}) -> {} {{\n", self.name, para, self.ret_ty)?;
        for expr in &self.body_stmts {
            write!(f, "  {};  #-> {}\n", &expr.0, &expr.1)?;
        }
        write!(f, "}}\n")
    }
}

impl<TY: Clone> Function_<TY> {
    pub fn fun_ty(&self, is_async: bool) -> FunTy {
        FunTy_ {
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

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.ty, self.name)
    }
}

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
            is_async: false,
            param_tys: vec![Ty::ChiikaEnv, Ty::Any],
            ret_ty: Box::new(Ty::RustFuture),
        })
    }
}

pub type FunTy = FunTy_<Ty>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunTy_<TY> {
    pub is_async: bool,
    pub param_tys: Vec<Ty>,
    pub ret_ty: Box<TY>,
}

impl<TY: fmt::Display> fmt::Display for FunTy_<TY> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let para = self
            .param_tys
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(",");
        write!(f, "FN(({})->{})", para, &self.ret_ty)
    }
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

type Typed<X, TY> = (X, TY);
type TypedExpr_<TY> = (Expr_<TY>, TY);
pub type TypedExpr = TypedExpr_<Ty>;
pub type Expr = Expr_<Ty>;

#[derive(Debug, Clone)]
pub enum Expr_<TY> {
    Number(i64),
    PseudoVar(PseudoVar),
    LVarRef(String),
    ArgRef(usize),
    FuncRef(String),
    OpCall(String, Box<TypedExpr_<TY>>, Box<TypedExpr_<TY>>),
    FunCall(Box<TypedExpr_<TY>>, Vec<TypedExpr_<TY>>),
    If(
        Box<TypedExpr_<TY>>,
        Vec<TypedExpr_<TY>>,
        Vec<TypedExpr_<TY>>,
    ),
    Yield(Box<TypedExpr_<TY>>),
    While(Box<TypedExpr_<TY>>, Vec<TypedExpr_<TY>>),
    Alloc(String),
    Assign(String, Box<TypedExpr_<TY>>),
    Return(Box<TypedExpr_<TY>>),
    Cast(CastType, Box<TypedExpr_<TY>>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PseudoVar {
    True,
    False,
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CastType {
    AnyToFun(FunTy),
    AnyToInt,
    IntToAny,
    FunToAny,
}

impl<TY> std::fmt::Display for Expr_<TY> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr_::Number(n) => write!(f, "{}", n),
            Expr_::PseudoVar(PseudoVar::True) => write!(f, "true"),
            Expr_::PseudoVar(PseudoVar::False) => write!(f, "false"),
            Expr_::PseudoVar(PseudoVar::Null) => write!(f, "null"),
            Expr_::LVarRef(name) => write!(f, "{}", name),
            Expr_::ArgRef(idx) => write!(f, "%arg_{}", idx),
            Expr_::FuncRef(name) => write!(f, "{}", name),
            Expr_::OpCall(op, lhs, rhs) => write!(f, "({} {} {})", lhs.0, op, rhs.0),
            Expr_::FunCall(func, args) => {
                write!(f, "{}(", func.0)?;
                for (i, arg) in args.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg.0)?;
                }
                write!(f, ")")
            }
            Expr_::If(cond, then, else_) => {
                write!(f, "if({}){{\n", cond.0)?;
                for stmt in then {
                    write!(f, "  {}\n", stmt.0)?;
                }
                write!(f, "}}")?;
                if !else_.is_empty() {
                    write!(f, " else {{\n")?;
                    for stmt in else_ {
                        write!(f, "  {}\n", stmt.0)?;
                    }
                    write!(f, "}}")?;
                }
                Ok(())
            }
            Expr_::Yield(e) => write!(f, "yield {}", e.0),
            Expr_::While(cond, body) => {
                write!(f, "while {} {{\n", cond.0)?;
                for stmt in body {
                    write!(f, "  {}\n", stmt.0)?;
                }
                write!(f, "}}")
            }
            Expr_::Alloc(name) => write!(f, "alloc {}", name),
            Expr_::Assign(name, e) => write!(f, "{} = {}", name, e.0),
            Expr_::Return(e) => write!(f, "return {}", e.0),
            Expr_::Cast(cast_type, e) => write!(f, "{:?}({})", cast_type, e.0),
        }
    }
}

impl<TY> Expr_<TY> {
    pub fn number(n: i64) -> TypedExpr {
        (Expr::Number(n), Ty::Int)
    }

    pub fn arg_ref(idx: usize, ty: Ty) -> TypedExpr {
        (Expr::ArgRef(idx), ty)
    }

    pub fn func_ref(name: impl Into<String>, fun_ty: FunTy) -> TypedExpr {
        (Expr::FuncRef(name.into()), fun_ty.into())
    }

    pub fn op_call(op: impl Into<String>, lhs: TypedExpr, rhs: TypedExpr) -> Result<TypedExpr> {
        let ty = match &op[..] {
            "+" | "-" | "*" | "/" => Ty::Int,
            "<" | "<=" | ">" | ">=" | "==" | "!=" => Ty::Bool,
            _ => return Err(anyhow!("[BUG] unknown operator: {op}")),
        };
        Ok((Expr::OpCall(op.into(), Box::new(lhs), Box::new(rhs)), ty))
    }

    pub fn fun_call(func: TypedExpr, args: Vec<TypedExpr>, result_ty: Ty) -> TypedExpr {
        (Expr::FunCall(Box::new(func), args), result_ty)
    }

    pub fn if_(cond: TypedExpr, then: Vec<TypedExpr>, else_: Vec<TypedExpr>) -> Result<TypedExpr> {
        if cond.1 != Ty::Bool {
            return Err(anyhow!("[BUG] if cond not bool: {:?}", cond));
        }
        let t1 = yielded_ty(&then);
        let t2 = yielded_ty(&else_);
        if t1 != t2 || t1.is_none() || t2.is_none() {
            return Err(anyhow!(
                "[BUG] if type invalid (t1: {:?}, t2: {:?})",
                t1,
                t2
            ));
        }
        Ok((Expr::If(Box::new(cond), then, else_), t1.unwrap()))
    }

    pub fn yield_(expr: TypedExpr) -> TypedExpr {
        let t = expr.1.clone();
        (Expr::Yield(Box::new(expr)), t)
    }

    pub fn yield_null() -> TypedExpr {
        let null = (Expr::PseudoVar(PseudoVar::Null), Ty::Null);
        (Expr::Yield(Box::new(null)), Ty::Null)
    }

    pub fn assign(name: impl Into<String>, e: TypedExpr) -> TypedExpr {
        (Expr::Assign(name.into(), Box::new(e)), Ty::Void)
    }

    pub fn return_(e: TypedExpr) -> TypedExpr {
        (Expr::Return(Box::new(e)), Ty::Void)
    }

    pub fn cast(e: TypedExpr, cast_type: CastType, ty: Ty) -> TypedExpr {
        (Expr::Cast(cast_type, Box::new(e)), ty)
    }
}

pub fn yielded_ty(stmts: &[TypedExpr]) -> Option<Ty> {
    stmts
        .last()
        .map(|stmt| match &stmt.0 {
            Expr::Yield(val) => Some(val.1.clone()),
            _ => None,
        })
        .flatten()
}
