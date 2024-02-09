pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);

pub type Program = Vec<Declaration>;

#[derive(PartialEq, Debug, Clone)]
pub enum Declaration {
    Extern(Spanned<Extern>),
    Function(Spanned<Function>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Extern {
    pub is_async: bool,
    pub name: String,
    pub params: Vec<Param>,
    pub ret_ty: Ty,
}

impl Extern {
    pub fn fun_ty(&self) -> FunTy {
        FunTy {
            is_async: self.is_async,
            param_tys: self.params.iter().map(|x| x.ty.clone()).collect::<Vec<_>>(),
            ret_ty: Box::new(self.ret_ty.clone()),
        }
    }

    pub fn into_empty_func(self) -> Function {
        Function {
            name: self.name,
            params: self.params,
            ret_ty: self.ret_ty,
            body_stmts: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub ret_ty: Ty,
    pub body_stmts: Vec<Expr>,
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

#[derive(PartialEq, Debug, Clone)]
pub struct Param {
    pub ty: Ty,
    pub name: String,
}

impl Param {
    pub fn new(ty: Ty, name: &str) -> Param {
        Param {
            ty,
            name: name.to_string(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Ty {
    Raw(String),
    Fun(FunTy),
}

impl Ty {
    pub fn raw(name: &str) -> Ty {
        Ty::Raw(name.to_string())
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FunTy {
    pub is_async: bool,
    pub param_tys: Vec<Ty>,
    pub ret_ty: Box<Ty>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Number(i64),
    VarRef(String),
    OpCall(String, Box<Expr>, Box<Expr>),
    FunCall(Box<Expr>, Vec<Expr>),
    Cast(Box<Expr>, Ty),
    Alloc(String),
    Assign(String, Box<Expr>),
    Return(Box<Expr>),
}

//pub fn to_source(ast: Vec<Declaration>) -> String {
//    ast.iter()
//        .map(|x| x.to_string())
//        .collect::<Vec<_>>()
//        .join("")
//}

impl Expr {
    pub fn var_ref(name: impl Into<String>) -> Expr {
        Expr::VarRef(name.into())
    }
}

//impl std::fmt::Display for Declaration {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        match self {
//            Declaration::Extern((x, _)) => write!(f, "{}", x),
//            Declaration::Function((x, _)) => write!(f, "{}", x),
//        }
//    }
//}
//
//impl std::fmt::Display for Extern {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        let params = self
//            .params
//            .iter()
//            .map(|x| x.to_string())
//            .collect::<Vec<_>>()
//            .join(", ");
//        write!(
//            f,
//            "extern {}({}) -> {};\n",
//            &self.name, params, &self.ret_ty
//        )
//    }
//}
//
//impl std::fmt::Display for Function {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        let params = self
//            .params
//            .iter()
//            .map(|x| x.to_string())
//            .collect::<Vec<_>>()
//            .join(", ");
//        write!(
//            f,
//            "func {}({}) -> {} {{\n",
//            &self.name, params, &self.ret_ty
//        )?;
//        for expr in &self.body_stmts {
//            write!(f, "  {};\n", expr)?;
//        }
//        write!(f, "}}\n")
//    }
//}
//
//impl std::fmt::Display for Param {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        write!(f, "{} {}", &self.ty, &self.name)
//    }
//}
//
//impl std::fmt::Display for Ty {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        match self {
//            Ty::Raw(s) => write!(f, "{}", s),
//            Ty::Fun(x) => {
//                let params = x
//                    .param_tys
//                    .iter()
//                    .map(|x| x.to_string())
//                    .collect::<Vec<_>>()
//                    .join(", ");
//                write!(f, "$FN(({}) -> {})", params, x.ret_ty)
//            }
//        }
//    }
//}
//
//impl std::fmt::Display for Expr {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        match self {
//            Expr::Number(n) => write!(f, "{}", n),
//            Expr::VarRef(s) => write!(f, "{}", s),
//            Expr::OpCall(op, l, r) => write!(f, "({} {} {})", l, op, r),
//            Expr::FunCall(fexpr, arg_exprs) => {
//                let args = arg_exprs
//                    .iter()
//                    .map(|x| x.to_string())
//                    .collect::<Vec<_>>()
//                    .join(", ");
//                write!(f, "{}({})", fexpr, args)
//            }
//            Expr::Cast(expr, ty) => write!(f, "($CAST({} as {}))", expr, ty),
//            Expr::Alloc(name) => write!(f, "alloc {}", name),
//            Expr::Assign(name, expr) => write!(f, "{} = {}", name, expr),
//        }
//    }
//}
