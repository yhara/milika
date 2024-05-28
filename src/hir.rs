pub mod asyncness_check;
pub mod blocked;
pub mod rewriter;
pub mod split;
pub mod typing;
pub mod untyped;
pub mod visitor;
use crate::ast;
use anyhow::{anyhow, Result};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Program {
    pub externs: Vec<Extern>,
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
            asyncness: self.is_async.into(),
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

#[derive(Debug, Clone)]
pub struct Function {
    pub generated: bool,
    pub asyncness: Asyncness,
    pub name: String,
    pub params: Vec<Param>,
    pub ret_ty: Ty,
    pub body_stmts: Vec<Typed<Expr>>,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let gen = if self.generated { "." } else { "" };
        let para = self
            .params
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(
            f,
            "fun{} {}{}({}) -> {} {{\n",
            gen, self.name, self.asyncness, para, self.ret_ty
        )?;
        for expr in &self.body_stmts {
            write!(f, "  {}  #-> {}\n", &expr.0, &expr.1)?;
        }
        write!(f, "}}\n")
    }
}

impl Function {
    pub fn fun_ty(&self) -> FunTy {
        FunTy {
            asyncness: self.asyncness.clone(),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Asyncness {
    Unknown,
    Sync,
    Async,
    Lowered,
}

impl fmt::Display for Asyncness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Asyncness::Unknown => write!(f, "[?]"),
            Asyncness::Sync => write!(f, "[+]"),
            Asyncness::Async => write!(f, "[*]"),
            Asyncness::Lowered => write!(f, ""), // "[.]"
        }
    }
}

impl From<bool> for Asyncness {
    fn from(x: bool) -> Self {
        if x {
            Asyncness::Async
        } else {
            Asyncness::Sync
        }
    }
}

impl Asyncness {
    pub fn is_async(&self) -> bool {
        match self {
            Asyncness::Unknown => panic!("[BUG] asyncness is unknown"),
            Asyncness::Async => true,
            Asyncness::Sync => false,
            Asyncness::Lowered => panic!("[BUG] asyncness is lost"),
        }
    }
}

type Typed<T> = (T, Ty);
pub type TypedExpr = Typed<Expr>;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    PseudoVar(PseudoVar),
    LVarRef(String),
    ArgRef(usize),
    FuncRef(String),
    OpCall(String, Box<Typed<Expr>>, Box<Typed<Expr>>),
    FunCall(Box<Typed<Expr>>, Vec<Typed<Expr>>),
    If(Box<Typed<Expr>>, Vec<Typed<Expr>>, Vec<Typed<Expr>>),
    Yield(Box<Typed<Expr>>),
    While(Box<Typed<Expr>>, Vec<Typed<Expr>>),
    Alloc(String),
    Assign(String, Box<Typed<Expr>>),
    Return(Box<Typed<Expr>>),
    Cast(CastType, Box<Typed<Expr>>),
    // Appears after `lower_async_if`
    CondReturn(
        Box<Typed<Expr>>,
        Box<Typed<Expr>>,
        Vec<Typed<Expr>>,
        Box<Typed<Expr>>,
        Vec<Typed<Expr>>,
    ),
    // Appears after `lower_if`
    Br(Box<Typed<Expr>>, usize),
    CondBr(Box<Typed<Expr>>, usize, usize),
    BlockArgRef,
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
    NullToAny,
    IntToAny,
    FunToAny,
}

impl CastType {
    pub fn result_ty(&self) -> Ty {
        match self {
            CastType::AnyToFun(x) => x.clone().into(),
            CastType::AnyToInt => Ty::Int,
            CastType::NullToAny | CastType::IntToAny | CastType::FunToAny => Ty::Any,
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::PseudoVar(PseudoVar::True) => write!(f, "true"),
            Expr::PseudoVar(PseudoVar::False) => write!(f, "false"),
            Expr::PseudoVar(PseudoVar::Null) => write!(f, "null"),
            Expr::LVarRef(name) => write!(f, "{}", name),
            Expr::ArgRef(idx) => write!(f, "%arg_{}", idx),
            Expr::FuncRef(name) => write!(f, "{}", name),
            Expr::OpCall(op, lhs, rhs) => write!(f, "({} {} {})", lhs.0, op, rhs.0),
            Expr::FunCall(func, args) => {
                let Ty::Fun(fun_ty) = &func.1 else {
                    panic!("[BUG] not a function: {:?}", func);
                };
                write!(f, "{}{}(", func.0, fun_ty.asyncness)?;
                for (i, arg) in args.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg.0)?;
                }
                write!(f, ")")
            }
            Expr::If(cond, then, else_) => {
                write!(f, "if({}){{\n", cond.0)?;
                for stmt in then {
                    write!(f, "    {}\n", stmt.0)?;
                }
                write!(f, "  }}")?;
                if !else_.is_empty() {
                    write!(f, " else {{\n")?;
                    for stmt in else_ {
                        write!(f, "    {}\n", stmt.0)?;
                    }
                    write!(f, "  }}")?;
                }
                Ok(())
            }
            Expr::Yield(e) => write!(f, "yield {}  # {}", e.0, e.1),
            Expr::While(cond, body) => {
                write!(f, "while {} {{\n", cond.0)?;
                for stmt in body {
                    write!(f, "  {}\n", stmt.0)?;
                }
                write!(f, "}}")
            }
            Expr::Alloc(name) => write!(f, "alloc {}", name),
            Expr::Assign(name, e) => write!(f, "{} = {}", name, e.0),
            Expr::Return(e) => write!(f, "return {}  # {}", e.0, e.1),
            Expr::Cast(cast_type, e) => write!(f, "({} as {})", e.0, cast_type.result_ty()),
            Expr::CondReturn(cond, fexpr_t, _args_t, fexpr_f, _args_f) => {
                let Ty::Fun(fun_ty_t) = &fexpr_t.1 else {
                    panic!("[BUG] not a function: {:?}", fexpr_t);
                };
                let Ty::Fun(fun_ty_f) = &fexpr_f.1 else {
                    panic!("[BUG] not a function: {:?}", fexpr_f);
                };
                write!(
                    f,
                    "cond_return {}, {}{}(...), {}{}(...)",
                    cond.0, fexpr_t.0, fun_ty_t.asyncness, fexpr_f.0, fun_ty_f.asyncness
                )
            }
            Expr::Br(e, target) => write!(f, "%br ^bb{}({})  # {}", target, e.0, e.1),
            Expr::CondBr(cond, target_t, target_f) => {
                write!(f, "%cond_br {} ^bb{} ^bb{}", cond.0, target_t, target_f)
            }
            Expr::BlockArgRef => write!(f, "%block_arg"),
        }
    }
}

impl Expr {
    pub fn number(n: i64) -> TypedExpr {
        (Expr::Number(n), Ty::Int)
    }

    //pub fn pseudo_var(pv: PseudoVar) -> TypedExpr {
    //    let t = match pv {
    //        PseudoVar::True | PseudoVar::False => Ty::Bool,
    //        PseudoVar::Null => Ty::Null,
    //    };
    //    (Expr::PseudoVar(pv), t)
    //}

    pub fn lvar_ref(name: impl Into<String>, ty: Ty) -> TypedExpr {
        (Expr::LVarRef(name.into()), ty)
    }

    pub fn arg_ref(idx: usize, ty: Ty) -> TypedExpr {
        (Expr::ArgRef(idx), ty)
    }

    pub fn func_ref(name: impl Into<String>, fun_ty: FunTy) -> TypedExpr {
        (Expr::FuncRef(name.into()), fun_ty.into())
    }

    pub fn op_call(op_: impl Into<String>, lhs: TypedExpr, rhs: TypedExpr) -> TypedExpr {
        let op = op_.into();
        let ty = match &op[..] {
            "+" | "-" | "*" | "/" => Ty::Int,
            "<" | "<=" | ">" | ">=" | "==" | "!=" => Ty::Bool,
            _ => panic!("[BUG] unknown operator: {op}"),
        };
        (Expr::OpCall(op, Box::new(lhs), Box::new(rhs)), ty)
    }

    pub fn fun_call(func: TypedExpr, args: Vec<TypedExpr>) -> TypedExpr {
        let result_ty = match &func.1 {
            Ty::Fun(f) => *f.ret_ty.clone(),
            _ => panic!("[BUG] not a function: {:?}", func),
        };
        (Expr::FunCall(Box::new(func), args), result_ty)
    }

    pub fn if_(cond: TypedExpr, then: Vec<TypedExpr>, else_: Vec<TypedExpr>) -> TypedExpr {
        if cond.1 != Ty::Bool {
            panic!("[BUG] if cond not bool: {:?}", cond);
        }
        let t1 = yielded_ty(&then);
        let t2 = yielded_ty(&else_);
        if t1 != t2 {
            panic!("[BUG] if types mismatch (t1: {:?}, t2: {:?})", t1, t2);
        }

        (Expr::If(Box::new(cond), then, else_), t1)
    }

    pub fn yield_(expr: TypedExpr) -> TypedExpr {
        let t = expr.1.clone();
        (Expr::Yield(Box::new(expr)), t)
    }

    pub fn yield_null() -> TypedExpr {
        let null = (Expr::PseudoVar(PseudoVar::Null), Ty::Null);
        (Expr::Yield(Box::new(null)), Ty::Null)
    }

    pub fn while_(cond: TypedExpr, body: Vec<TypedExpr>) -> TypedExpr {
        if cond.1 != Ty::Bool {
            panic!("[BUG] while cond not bool: {:?}", cond);
        }
        (Expr::While(Box::new(cond), body), Ty::Null)
    }

    pub fn assign(name: impl Into<String>, e: TypedExpr) -> TypedExpr {
        (Expr::Assign(name.into(), Box::new(e)), Ty::Void)
    }

    pub fn return_(e: TypedExpr) -> TypedExpr {
        (Expr::Return(Box::new(e)), Ty::Void)
    }

    pub fn cast(cast_type: CastType, e: TypedExpr) -> TypedExpr {
        let ty = match &cast_type {
            CastType::AnyToFun(f) => f.clone().into(),
            CastType::AnyToInt => Ty::Int,
            CastType::NullToAny => Ty::Any,
            CastType::IntToAny => Ty::Any,
            CastType::FunToAny => Ty::Any,
        };
        (Expr::Cast(cast_type, Box::new(e)), ty)
    }

    pub fn cond_return(
        cond: TypedExpr,
        fexpr_t: TypedExpr,
        args_t: Vec<TypedExpr>,
        fexpr_f: TypedExpr,
        args_f: Vec<TypedExpr>,
    ) -> TypedExpr {
        (
            Expr::CondReturn(
                Box::new(cond),
                Box::new(fexpr_t),
                args_t,
                Box::new(fexpr_f),
                args_f,
            ),
            Ty::Void,
        )
    }

    pub fn br(value: TypedExpr, target: usize) -> TypedExpr {
        (Expr::Br(Box::new(value), target), Ty::Void)
    }

    pub fn cond_br(cond: TypedExpr, target_t: usize, target_f: usize) -> TypedExpr {
        (Expr::CondBr(Box::new(cond), target_t, target_f), Ty::Void)
    }

    pub fn block_arg_ref(ty: Ty) -> TypedExpr {
        (Expr::BlockArgRef, ty)
    }

    pub fn is_async_fun_call(&self) -> bool {
        match self {
            Expr::FunCall(fexpr, _args) => fexpr.1.is_async_fun(),
            _ => false,
        }
    }
}

pub fn yielded_ty(stmts: &[TypedExpr]) -> Ty {
    let stmt = stmts.last().unwrap();
    match &stmt.0 {
        Expr::Yield(val) => val.1.clone(),
        Expr::Return(_) => Ty::Void,
        _ => panic!("[BUG] if branch not terminated with yield: {:?}", stmt),
    }
}
