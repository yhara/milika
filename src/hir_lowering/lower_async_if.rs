use crate::hir;
use anyhow::{anyhow, Result};
use std::collections::VecDeque;

#[derive(Debug)]
struct Chapter {
    name: String,
    stmts: Vec<hir::TypedExpr>,
}

impl Chapter {
    fn new(name: String) -> Chapter {
        Chapter {
            name,
            stmts: vec![],
        }
    }

    fn new_suffixed(base_name: &str, suffix: &str) -> Chapter {
        Chapter::new(format!("{}'{}", base_name, suffix))
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
    fn new(first_chap_name: &str) -> Chapters {
        Chapters {
            chaps: VecDeque::from(vec![Chapter::new(first_chap_name.to_string())]),
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

pub fn run(hir: hir::Program) -> Result<hir::Program> {
    let mut funcs = vec![];
    for f in hir.funcs {
        let mut split_funcs = compile_func(f)?;
        funcs.append(&mut split_funcs);
    }
    Ok(hir::Program { funcs, ..hir })
}

fn compile_func(mut f: hir::Function) -> Result<Vec<hir::Function>> {
    let body_stmts = f.body_stmts.drain(..).collect::<Vec<_>>();
    let chaps = {
        let mut lower = LowerAsyncIf {
            chapters: Chapters::new(&f.name),
            orig_func: &f,
            allocs: hir::visitor::Allocs::collect(&f.body_stmts)?,
        };

        for expr in body_stmts {
            let new_expr = lower.compile_expr(expr)?;
            lower.chapters.add_stmt(new_expr);
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
    fn compile_expr(&mut self, e: hir::TypedExpr) -> Result<hir::TypedExpr> {
        let new_e = match e.0 {
            hir::Expr::Number(_) => e,
            hir::Expr::PseudoVar(_) => e,
            hir::Expr::LVarRef(_) => todo!(),
            hir::Expr::Assign(name, rhs) => hir::Expr::assign(name, self.compile_expr(*rhs)?),
            hir::Expr::ArgRef(_) => e,
            hir::Expr::FuncRef(_) => e,
            hir::Expr::OpCall(op, lhs, rhs) => {
                hir::Expr::op_call(op, self.compile_expr(*lhs)?, self.compile_expr(*rhs)?, e.1)
            }
            hir::Expr::FunCall(fexpr, arg_exprs) => hir::Expr::fun_call(
                self.compile_expr(*fexpr)?,
                arg_exprs
                    .into_iter()
                    .map(|expr| self.compile_expr(expr))
                    .collect::<Result<_>>()?,
                e.1,
            ),
            hir::Expr::While(cond_expr, body_exprs) => hir::Expr::while_(
                self.compile_expr(*cond_expr)?,
                body_exprs
                    .into_iter()
                    .map(|expr| self.compile_expr(expr))
                    .collect::<Result<_>>()?,
            ),
            hir::Expr::Alloc(_) => e,
            hir::Expr::Return(expr) => hir::Expr::return_(self.compile_expr(*expr)?),
            hir::Expr::Cast(_, _) => {
                return Err(anyhow!(
                    "[BUG] cast should not appear before lower_async_if"
                ))
            }
            hir::Expr::Para(_) => todo!(),
            hir::Expr::If(cond_expr, then_exprs, else_exprs) => {
                self.compile_if(*cond_expr, then_exprs, else_exprs)?
            }
            hir::Expr::ValuedIf(_, _, _) => todo!(),
            hir::Expr::Yield(_) => todo!(),
        };
        Ok(new_e)
    }

    fn compile_if(
        &mut self,
        cond_expr: hir::TypedExpr,
        then_exprs: Vec<hir::TypedExpr>,
        else_exprs: Vec<hir::TypedExpr>,
    ) -> Result<hir::TypedExpr> {
        let chap_fun_ty = self.chapter_fun_ty();
        let func_name = self.chapters.current_name().to_string();

        let new_cond_expr = self.compile_expr(cond_expr)?;
        let mut then_chap = Chapter::new_suffixed(&func_name, "t");
        let mut else_chap = Chapter::new_suffixed(&func_name, "f");
        // Statements after `if` goes to an "endif" chapter
        let endif_chap = Chapter::new_suffixed(&func_name, "e"); // e for endif

        let then_ret = hir::Expr::yield_(self.goto_call(&then_chap.name, chap_fun_ty.clone()));
        let else_ret = hir::Expr::yield_(self.goto_call(&else_chap.name, chap_fun_ty.clone()));
        let goto_endif = hir::Expr::return_(self.goto_call(&endif_chap.name, chap_fun_ty));

        self.compile_clause(&mut then_chap, then_exprs, goto_endif.clone())?;
        self.compile_clause(&mut else_chap, else_exprs, goto_endif)?;

        let terminator = hir::Expr::return_(hir::Expr::valued_if(
            new_cond_expr,
            vec![then_ret],
            vec![else_ret],
        )?);
        self.chapters.add_stmt(terminator);
        self.chapters.add(then_chap);
        self.chapters.add(else_chap);
        self.chapters.add(endif_chap);

        Ok(hir::Expr::pseudo_var(hir::PseudoVar::Null))
    }

    fn compile_clause(
        &mut self,
        clause_chap: &mut Chapter,
        exprs: Vec<hir::TypedExpr>,
        goto_endif: hir::TypedExpr,
    ) -> Result<()> {
        for expr in exprs {
            let new_expr = self.compile_expr(expr)?;
            clause_chap.add_stmt(new_expr);
        }
        clause_chap.add_stmt(goto_endif);
        Ok(())
    }

    /// Generate a call to the chapter function
    fn goto_call(&self, chap_name: &str, chap_fun_ty: hir::FunTy) -> hir::TypedExpr {
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
        hir::Expr::fun_call(
            hir::Expr::func_ref(chap_name, chap_fun_ty),
            args,
            self.orig_func.ret_ty.clone(),
        )
    }

    fn chapter_fun_ty(&mut self) -> hir::FunTy {
        let mut param_tys = self.orig_func.fun_ty(false).param_tys.clone();
        param_tys.extend(self.allocs.iter().map(|(_, ty)| ty.clone()));
        hir::FunTy {
            is_async: false,
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
        new_f.body_stmts = chap.stmts;
        funcs.push(new_f);
    }
    funcs
}
