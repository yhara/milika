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
        let t = x.ret_ty.clone().try_into()?;
        Ok(Self {
            is_internal: x.is_internal,
            name: x.name.clone(),
            params: x
                .params
                .iter()
                .map(|x| x.clone().try_into())
                .collect::<Result<_>>()?,
            ret_ty: if x.is_async {
                Ty::Async(Box::new(t))
            } else {
                t
            },
        })
    }

    pub fn fun_ty(&self) -> FunTy {
        FunTy {
            param_tys: self.params.iter().map(|x| x.ty.clone()).collect::<Vec<_>>(),
            ret_ty: Box::new(self.ret_ty.clone()),
        }
    }
}

impl fmt::Display for Extern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inte = if self.is_internal { "(internal)" } else { "" };
        let para = self
            .params
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(
            f,
            "extern{} {}({}) -> {};\n",
            inte, self.name, para, self.ret_ty
        )
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub ret_ty: Ty,
    pub body_stmts: Vec<Typed<Expr>>,
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
        for expr in &self.body_stmts {
            write!(f, "  {};  #-> {}\n", &expr.0, &expr.1)?;
        }
        write!(f, "}}\n")
    }
}

impl Function {
    pub fn fun_ty(&self) -> FunTy {
        FunTy {
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
    Null, // A unit type. Represented by `i64 0`
    Void, // eg. the type of `return` or assignment. There is no value of this type.
    Any,  // Corresponds to `ptr` in llvm
    ChiikaEnv,
    RustFuture,
    Int,
    Bool,
    Fun(FunTy),
    Async(Box<Ty>),
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
            param_tys: vec![Ty::ChiikaEnv, Ty::Any],
            ret_ty: Box::new(Ty::RustFuture),
        })
    }

    pub fn is_async(&self) -> bool {
        matches!(self, Ty::Async(_))
    }

    pub fn async_result_ty(&self) -> Option<&Ty> {
        match self {
            Ty::Async(t) => Some(t),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunTy {
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
        let orig_t = f.ret_ty.clone().try_into()?;
        let t = if is_async {
            Ty::Async(Box::new(orig_t))
        } else {
            orig_t
        };
        Ok(Self {
            param_tys: f
                .params
                .iter()
                .map(|x| x.ty.clone().try_into())
                .collect::<Result<_>>()?,
            ret_ty: Box::new(t),
        })
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
    ValuedIf(Box<Typed<Expr>>, Vec<Typed<Expr>>, Vec<Typed<Expr>>),
    Yield(Box<Typed<Expr>>),
    While(Box<Typed<Expr>>, Vec<Typed<Expr>>),
    Alloc(String),
    Assign(String, Box<Typed<Expr>>),
    Return(Box<Typed<Expr>>),
    Cast(CastType, Box<Typed<Expr>>),
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
                write!(f, "{}(", func.0)?;
                for (i, arg) in args.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg.0)?;
                    //write!(f, "{}: {}", arg.0, arg.1)?;
                }
                write!(f, ")")
            }
            Expr::If(cond, then, else_) | Expr::ValuedIf(cond, then, else_) => {
                write!(f, "if({}){{\n", cond.0)?;
                for stmt in then {
                    write!(f, "    {}  #-> {}\n", stmt.0, stmt.1)?;
                }
                write!(f, "  }}")?;
                if !else_.is_empty() {
                    write!(f, " else {{\n")?;
                    for stmt in else_ {
                        write!(f, "    {}  #-> {}\n", stmt.0, stmt.1)?;
                    }
                    write!(f, "  }}")?;
                }
                Ok(())
            }
            Expr::Yield(e) => write!(f, "yield {}", e.0),
            Expr::While(cond, body) => {
                write!(f, "while {} {{\n", cond.0)?;
                for stmt in body {
                    write!(f, "  {}\n", stmt.0)?;
                }
                write!(f, "}}")
            }
            Expr::Alloc(name) => write!(f, "alloc {}", name),
            Expr::Assign(name, e) => write!(f, "{} = {}", name, e.0),
            Expr::Return(e) => write!(f, "return {}", e.0),
            Expr::Cast(cast_type, e) => write!(f, "{:?}({})", cast_type, e.0),
        }
    }
}

impl Expr {
    pub fn number(n: i64) -> TypedExpr {
        (Expr::Number(n), Ty::Int)
    }

    pub fn pseudo_var(pv: PseudoVar) -> TypedExpr {
        let t = if pv == PseudoVar::Null {
            Ty::Null
        } else {
            Ty::Bool
        };
        (Expr::PseudoVar(pv), t)
    }

    pub fn lvar_ref(name: impl Into<String>, ty: Ty) -> TypedExpr {
        (Expr::LVarRef(name.into()), ty)
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

    pub fn if_(cond: TypedExpr, then: Vec<TypedExpr>, else_: Vec<TypedExpr>) -> TypedExpr {
        (Expr::If(Box::new(cond), then, else_), Ty::Void)
    }

    pub fn valued_if(
        cond: TypedExpr,
        then: Vec<TypedExpr>,
        else_: Vec<TypedExpr>,
    ) -> Result<TypedExpr> {
        let t1 = if let Some((Expr::Yield(e), _)) = then.last() {
            e.1.clone()
        } else {
            return Err(anyhow!("The last statement of then branch must be `yield`"));
        };
        let t2 = if let Some((Expr::Yield(e), _)) = else_.last() {
            e.1.clone()
        } else {
            return Err(anyhow!("The last statement of else branch must be `yield`"));
        };
        if t1 != t2 {
            return Err(anyhow!(
                "The types of then and else branches must be the same ({} != {})",
                t1,
                t2
            ));
        }
        Ok((Expr::ValuedIf(Box::new(cond), then, else_), t1))
    }

    pub fn yield_(e: TypedExpr) -> TypedExpr {
        let t = e.1.clone();
        (Expr::Yield(Box::new(e)), t)
    }

    pub fn while_(cond: TypedExpr, body: Vec<TypedExpr>) -> TypedExpr {
        (Expr::While(Box::new(cond), body), Ty::Void)
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
