use nom_locate;
pub type Span<'a> = nom_locate::LocatedSpan<&'a str>;
pub type Spanned<'a, T> = (T, Span<'a>);

pub type Program<'a> = Spanned<'a, Vec<Declaration<'a>>>;

#[derive(PartialEq, Debug, Clone)]
pub enum Declaration<'a> {
    Extern(Spanned<'a, Extern>),
    Function(Spanned<'a, Function<'a>>),
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

    //pub fn into_empty_func(self) -> Function {
    //    Function {
    //        name: self.name,
    //        params: self.params,
    //        ret_ty: self.ret_ty,
    //        body_stmts: Default::default(),
    //    }
    //}
}

#[derive(PartialEq, Debug, Clone)]
pub struct Function<'a> {
    pub name: String,
    pub params: Vec<Param>,
    pub ret_ty: Ty,
    pub body_stmts: Vec<SpannedExpr<'a>>,
}

impl<'a> Function<'a> {
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
    //pub fn new(ty: Ty, name: &str) -> Param {
    //    Param {
    //        ty,
    //        name: name.to_string(),
    //    }
    //}
}

#[derive(PartialEq, Debug, Clone)]
pub enum Ty {
    Raw(String),
    Fun(FunTy),
}

impl Ty {
    //pub fn raw(name: &str) -> Ty {
    //    Ty::Raw(name.to_string())
    //}
}

#[derive(PartialEq, Debug, Clone)]
pub struct FunTy {
    pub is_async: bool,
    pub param_tys: Vec<Ty>,
    pub ret_ty: Box<Ty>,
}

pub type SpannedExpr<'a> = Spanned<'a, Expr<'a>>;

#[derive(PartialEq, Debug, Clone)]
pub enum Expr<'a> {
    Number(i64),
    VarRef(String),
    OpCall(String, Box<SpannedExpr<'a>>, Box<SpannedExpr<'a>>),
    FunCall(Box<SpannedExpr<'a>>, Vec<SpannedExpr<'a>>),
    If(
        Box<SpannedExpr<'a>>,
        Vec<SpannedExpr<'a>>,
        Option<Vec<SpannedExpr<'a>>>,
    ),
    While(Box<SpannedExpr<'a>>, Vec<SpannedExpr<'a>>),
    Cast(Box<SpannedExpr<'a>>, Ty),
    Alloc(String),
    Assign(String, Box<SpannedExpr<'a>>),
    Return(Box<SpannedExpr<'a>>),
    Para(Vec<SpannedExpr<'a>>),
}
