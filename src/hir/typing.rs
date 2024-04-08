use crate::hir;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

struct Typing<'f> {
    sigs: HashMap<String, hir::FunTy>,
    current_func_name: Option<&'f String>,
    current_func_params: Option<&'f [hir::Param]>,
    current_func_ret_ty: Option<&'f hir::Ty>,
}

/// Create typed HIR from untyped HIR.
pub fn run(hir: hir::Program_<()>) -> Result<hir::Program> {
    let mut c = Typing {
        sigs: HashMap::new(),
        current_func_name: None,
        current_func_params: None,
        current_func_ret_ty: None,
    };
    for e in &hir.externs {
        c.sigs.insert(e.name.clone(), e.fun_ty());
    }
    for f in &hir.funcs {
        c.sigs.insert(f.name.clone(), f.fun_ty(false));
    }

    let funcs = hir
        .funcs
        .into_iter()
        .map(|f| c.compile_func(f))
        .collect::<Result<_>>()?;

    Ok(hir::Program {
        externs: hir.externs,
        funcs,
    })
}

impl<'f> Typing<'f> {
    fn compile_func(&mut self, func: hir::Function_<()>) -> Result<hir::Function> {
        self.current_func_name = Some(&func.name);
        self.current_func_params = Some(&func.params);
        self.current_func_ret_ty = Some(&func.ret_ty);
        let mut lvars = HashMap::new();
        let body_stmts = func
            .body_stmts
            .into_iter()
            .map(|e| self.compile_expr(&mut lvars, e))
            .collect::<Result<_>>()?;
        Ok(hir::Function_ {
            name: func.name,
            params: func.params,
            ret_ty: func.ret_ty,
            body_stmts,
        })
    }

    fn compile_expr(
        &mut self,
        lvars: &mut HashMap<String, hir::Ty>,
        e: hir::TypedExpr_<()>,
    ) -> Result<hir::TypedExpr> {
        let new_e = match e.0 {
            hir::Expr_::Number(_) => (e.0, hir::Ty::Int),
            hir::Expr_::PseudoVar(hir::PseudoVar::True) => (e.0, hir::Ty::Bool),
            hir::Expr_::PseudoVar(hir::PseudoVar::False) => (e.0, hir::Ty::Bool),
            hir::Expr_::PseudoVar(hir::PseudoVar::Null) => (e.0, hir::Ty::Null),
            hir::Expr_::LVarRef(name) => {
                if let Some(ty) = lvars.get(&name) {
                    (e.0, ty.clone())
                } else {
                    return Err(anyhow!("[BUG] unknown variable `{name}'"));
                }
            }
            hir::Expr_::ArgRef(i) => {
                let t = self.current_func_params.unwrap()[i].ty.clone();
                (e.0, t)
            }
            hir::Expr_::FuncRef(name) => {
                if let Some(fun_ty) = self.sigs.get(&name) {
                    (e.0, hir::Ty::Fun(fun_ty.clone()))
                } else {
                    return Err(anyhow!("[BUG] unknown function `{name}'"));
                }
            }
            hir::Expr_::OpCall(op, l, r) => {
                let ll = self.compile_expr(lvars, *l)?;
                let rr = self.compile_expr(lvars, *r)?;
                hir::Expr::op_call(op, ll, rr)?
            }
            hir::Expr_::FunCall(fexpr, arg_exprs) => {
                let new_fexpr = self.compile_expr(lvars, *fexpr)?;
                let hir::Ty::Fun(fun_ty) = &new_fexpr.1 else {
                    return Err(anyhow!("not a function: {:?}", fexpr));
                };
                if fun_ty.param_tys.len() != arg_exprs.len() {
                    return Err(anyhow!(
                        "funcall arity mismatch (expected {}, got {}): {:?}",
                        fun_ty.param_tys.len(),
                        arg_exprs.len(),
                        e
                    ));
                }
                let new_arg_exprs = arg_exprs
                    .into_iter()
                    .map(|e| self.compile_expr(lvars, e))
                    .collect::<Result<_>>()?;
                hir::Expr::fun_call(new_fexpr, new_arg_exprs)?
            }
            hir::Expr_::If(cond, then, els) => {
                self.compile_expr(lvars, *cond)?;
                if cond.1 != hir::Ty::Bool {
                    return Err(anyhow!("condition should be bool but got {:?}", cond.1));
                }
                self.compile_exprs(lvars, then)?;
                self.compile_exprs(lvars, els)?;
                let t1 = hir::yielded_ty(&then).unwrap();
                let t2 = hir::yielded_ty(&els).unwrap();
                if t1 != t2 {
                    return Err(anyhow!(
                        "then and else should have the same type but got {:?} and {:?}",
                        t1,
                        t2
                    ));
                }
                (e.0, t1.clone())
            }
            hir::Expr_::Yield(val) => {
                self.compile_expr(lvars, val)?;
                (e.0, val.1.clone())
            }
            hir::Expr_::While(cond, body) => {
                self.compile_expr(lvars, cond)?;
                self.compile_exprs(lvars, body)?;
                (e.0, hir::Ty::Void)
            }
            hir::Expr_::Alloc(name) => {
                // Milika vars are always Int now
                lvars.insert(name.clone(), hir::Ty::Int);
                (e.0, hir::Ty::Void)
            }
            hir::Expr_::Assign(_, val) => {
                self.compile_expr(lvars, val)?;
                (e.0, hir::Ty::Void)
            }
            hir::Expr_::Return(val) => {
                self.compile_expr(lvars, val)?;
                if val.1 != *self.current_func_ret_ty.unwrap() {
                    return Err(anyhow!(
                        "return type mismatch: {} should return {:?} but got {:?}",
                        self.current_func_name.unwrap(),
                        self.current_func_ret_ty.unwrap(),
                        val.1
                    ));
                }
                (e.0, hir::Ty::Void)
            }
            hir::Expr_::Cast(_, _) => {
                return Err(anyhow!("[BUG] Cast unexpected here"));
            }
        };
        Ok(new_e)
    }

    fn compile_exprs(
        &mut self,
        lvars: &mut HashMap<String, hir::Ty>,
        es: Vec<hir::TypedExpr_<()>>,
    ) -> Result<Vec<hir::TypedExpr>> {
        es.into_iter()
            .map(|e| self.compile_expr(lvars, e))
            .collect()
    }
}
