//! Split `if` statements with (possible) async call into multiple functions.
//!
//! Example:
//! ```
//! // Before
//! fun foo() {
//!   ...
//!   x = if (a) {
//!     b ...
//!     yield c
//!   } else {
//!     d ...
//!     yield e
//!   }
//!   ...
//!   x + ...
//!
//! // After
//! fun foo() -> Foo {
//!   ...
//!     cond_return a, foo't(), foo'f()
//! }
//! fun foo't() -> Foo {
//!     b ...
//!     return foo'e(c)
//! }
//! fun foo'f() -> Foo {
//!     d ...
//!     return foo'e(e)
//! }
//! fun foo'e(x) -> Foo {
//!     ...
//!     x + ...
//! }
//! ```
use crate::hir;
use anyhow::{anyhow, Result};
use std::collections::VecDeque;

pub fn run(hir: hir::Program) -> Result<hir::split::Program> {
    let mut funcs = vec![];
    for f in hir.funcs {
        funcs.push(compile_func(f)?);
    }
    Ok(hir::split::Program {
        externs: hir.externs,
        funcs,
    })
}

#[derive(Debug)]
struct Chapter {
    name: String,
    params: Vec<hir::Param>,
    stmts: Vec<hir::TypedExpr>,
}

impl Chapter {
    fn new(name: String, params: Vec<hir::Param>) -> Chapter {
        Chapter {
            name,
            params,
            stmts: vec![],
        }
    }

    fn new_suffixed(base_name: &str, suffix: &str, params: Vec<hir::Param>) -> Chapter {
        Chapter::new(format!("{}'{}", base_name, suffix), params)
    }

    fn add_stmt(&mut self, stmt: hir::TypedExpr) {
        self.stmts.push(stmt);
    }
}

#[derive(Debug)]
struct Chapters {
    chaps: VecDeque<Chapter>,
}

impl Chapters {
    fn new() -> Chapters {
        Chapters {
            chaps: VecDeque::new(),
        }
    }

    /// Returns the name of the last chapter
    fn current_name(&self) -> &str {
        &self.chaps.back().unwrap().name
    }

    fn add(&mut self, chap: Chapter) {
        self.chaps.push_back(chap);
    }

    fn add_stmt(&mut self, stmt: hir::TypedExpr) {
        self.chaps.back_mut().unwrap().add_stmt(stmt);
    }
}

fn compile_func(mut f: hir::Function) -> Result<Vec<hir::Function>> {
    let body_stmts = f.body_stmts.drain(..).collect::<Vec<_>>();
    let chaps = {
        let mut lower = LowerAsyncIf {
            chapters: Chapters::new(),
            orig_func: &f,
            allocs: hir::visitor::Allocs::collect(&f.body_stmts)?,
        };
        let first_chap = Chapter::new(f.name.clone(), f.params.clone());
        lower.chapters.add(first_chap);

        for expr in body_stmts {
            if let Some(new_expr) = lower.compile_expr(expr)? {
                lower.chapters.add_stmt(new_expr);
            }
        }

        lower.chapters
    };
    Ok(serialize_chapters(f, chaps))
}

struct LowerAsyncIf<'a> {
    chapters: Chapters,
    orig_func: &'a hir::Function,
    allocs: Vec<(String, hir::Ty)>,
}

impl<'a> LowerAsyncIf<'a> {
    fn compile_value_expr(&mut self, e: hir::TypedExpr) -> Result<hir::TypedExpr> {
        if let Some(expr) = self.compile_expr(e)? {
            Ok(expr)
        } else {
            Err(anyhow!("[BUG] unexpected void expr"))
        }
    }

    fn compile_expr(&mut self, e: hir::TypedExpr) -> Result<Option<hir::TypedExpr>> {
        let new_e = match e.0 {
            hir::Expr::Number(_) => e,
            hir::Expr::PseudoVar(_) => e,
            hir::Expr::LVarRef(_) => e,
            hir::Expr::Assign(name, rhs) => hir::Expr::assign(name, self.compile_value_expr(*rhs)?),
            hir::Expr::ArgRef(_) => e,
            hir::Expr::FuncRef(_) => e,
            hir::Expr::OpCall(op, lhs, rhs) => hir::Expr::op_call(
                op,
                self.compile_value_expr(*lhs)?,
                self.compile_value_expr(*rhs)?,
            ),
            hir::Expr::FunCall(fexpr, arg_exprs) => hir::Expr::fun_call(
                self.compile_value_expr(*fexpr)?,
                arg_exprs
                    .into_iter()
                    .map(|expr| self.compile_value_expr(expr))
                    .collect::<Result<_>>()?,
            ),
            hir::Expr::While(cond_expr, body_exprs) => hir::Expr::while_(
                self.compile_value_expr(*cond_expr)?,
                body_exprs
                    .into_iter()
                    .map(|expr| self.compile_value_expr(expr))
                    .collect::<Result<_>>()?,
            ),
            hir::Expr::Alloc(_) => e,
            hir::Expr::Return(expr) => hir::Expr::return_(self.compile_value_expr(*expr)?),
            hir::Expr::If(cond_expr, then_exprs, else_exprs) => {
                return self.compile_if(&e.1, *cond_expr, then_exprs, else_exprs);
            }
            hir::Expr::Yield(expr) => hir::Expr::yield_(self.compile_value_expr(*expr)?),
            _ => {
                panic!("[BUG] unexpected expr in lower_async_if: {:?}", e.0)
            }
        };
        Ok(Some(new_e))
    }

    fn compile_if(
        &mut self,
        if_ty: &hir::Ty,
        cond_expr: hir::TypedExpr,
        then_exprs: Vec<hir::TypedExpr>,
        else_exprs: Vec<hir::TypedExpr>,
    ) -> Result<Option<hir::TypedExpr>> {
        let func_name = self.chapters.current_name().to_string();

        let new_cond_expr = self.compile_value_expr(cond_expr)?;
        let mut then_chap = Chapter::new_suffixed(&func_name, "t", self.orig_func.params.clone());
        let mut else_chap = Chapter::new_suffixed(&func_name, "f", self.orig_func.params.clone());
        // Statements after `if` goes to an "endif" chapter
        let mut endif_params = self.orig_func.params.clone();
        endif_params.push(hir::Param {
            name: "$ifResult".to_string(),
            ty: if_ty.clone(),
        });
        let endif_chap = Chapter::new_suffixed(&func_name, "e", endif_params); // e for endif

        self.compile_clause(&mut then_chap, then_exprs, &endif_chap.name)?;
        self.compile_clause(&mut else_chap, else_exprs, &endif_chap.name)?;

        let (fexpr_t, args_t) = self.goto_call(&then_chap.name, None);
        let (fexpr_f, args_f) = self.goto_call(&else_chap.name, None);
        let terminator = hir::Expr::cond_return(new_cond_expr, fexpr_t, args_t, fexpr_f, args_f);
        self.chapters.add_stmt(terminator);
        self.chapters.add(then_chap);
        self.chapters.add(else_chap);
        if *if_ty == hir::Ty::Void {
            // Both branches end with return
            Ok(None)
        } else {
            self.chapters.add(endif_chap);
            Ok(Some(hir::Expr::arg_ref(
                self.orig_func.params.len(),
                if_ty.clone(),
            )))
        }
    }

    fn compile_clause(
        &mut self,
        clause_chap: &mut Chapter,
        mut exprs: Vec<hir::TypedExpr>,
        endif_chap_name: &str,
    ) -> Result<()> {
        let e = exprs.pop().unwrap();
        let opt_vexpr = match e {
            (hir::Expr::Return(_), _) => {
                exprs.push(e);
                None
            }
            (hir::Expr::Yield(vexpr), _) => Some(vexpr),
            _ => {
                return Err(anyhow!(
                    "[BUG] last statement of a clause must be a yield or a return"
                ))
            }
        };
        for expr in exprs {
            if let Some(new_expr) = self.compile_expr(expr)? {
                clause_chap.add_stmt(new_expr);
            }
        }
        if let Some(vexpr) = opt_vexpr {
            let new_vexpr = self.compile_value_expr(*vexpr)?;
            let (fexpr, args) = self.goto_call(&endif_chap_name, Some(new_vexpr));
            let goto_endif = hir::Expr::return_(hir::Expr::fun_call(fexpr, args));
            clause_chap.add_stmt(goto_endif);
        }
        Ok(())
    }

    /// Generate a call to the chapter function
    fn goto_call(
        &self,
        chap_name: &str,
        to_endif: Option<hir::TypedExpr>,
    ) -> (hir::TypedExpr, Vec<hir::TypedExpr>) {
        let mut args = self
            .orig_func
            .params
            .iter()
            .enumerate()
            .map(|(i, param)| hir::Expr::arg_ref(i, param.ty.clone()))
            .collect::<Vec<_>>();
        args.extend(
            self.allocs
                .iter()
                .map(|(name, ty)| hir::Expr::lvar_ref(name.clone(), ty.clone())),
        );
        let mut t = None;
        if let Some(expr) = to_endif {
            t = Some(expr.1.clone());
            args.push(expr);
        }
        let chap_fun_ty = self.chapter_fun_ty(t);
        (hir::Expr::func_ref(chap_name, chap_fun_ty), args)
    }

    fn chapter_fun_ty(&self, endif: Option<hir::Ty>) -> hir::FunTy {
        let mut param_tys = self.orig_func.fun_ty().param_tys.clone();
        param_tys.extend(self.allocs.iter().map(|(_, ty)| ty.clone()));
        if let Some(t) = endif {
            param_tys.push(t);
        }
        hir::FunTy {
            asyncness: self.orig_func.asyncness.clone(),
            param_tys,
            ret_ty: Box::new(self.orig_func.ret_ty.clone()),
        }
    }
}

fn serialize_chapters(f: hir::Function, chapters: Chapters) -> Vec<hir::Function> {
    let mut funcs = vec![];
    for chap in chapters.chaps {
        let mut new_f = f.clone();
        new_f.name = chap.name;
        new_f.params = chap.params;
        new_f.body_stmts = chap.stmts;
        funcs.push(new_f);
    }
    funcs
}
