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
    // Denotes the rust-implemented function returns Future
    pub is_async: bool,
    // Used in prelude.rs
    pub is_internal: bool,
    pub name: String,
    pub params: Vec<Param>,
    pub ret_ty: Ty,
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

#[derive(PartialEq, Debug, Clone)]
pub enum Ty {
    Raw(String),
    //Fun(FunTy),
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
    Alloc(String),
    Assign(String, Box<SpannedExpr<'a>>),
    Return(Box<SpannedExpr<'a>>),
    Para(Vec<SpannedExpr<'a>>),
}
