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
pub fn run(hir: &mut hir::Program) -> Result<()> {
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

    for f in hir.funcs.iter_mut() {
        c.compile_func(f)?;
    }

    Ok(())
}

impl<'f> Typing<'f> {
    fn compile_func(&mut self, func: &'f mut hir::Function) -> Result<()> {
        self.current_func_name = Some(&func.name);
        self.current_func_params = Some(&func.params);
        self.current_func_ret_ty = Some(&func.ret_ty);
        let mut lvars = HashMap::new();
        func.body_stmts
            .iter_mut()
            .try_for_each(|e| self.compile_expr(&mut lvars, e))?;
        Ok(())
    }

    fn compile_expr(
        &mut self,
        lvars: &mut HashMap<String, hir::Ty>,
        e: &mut hir::TypedExpr,
    ) -> Result<()> {
        match &mut e.0 {
            hir::Expr::Number(_) => e.1 = hir::Ty::Int,
            hir::Expr::PseudoVar(hir::PseudoVar::True) => e.1 = hir::Ty::Bool,
            hir::Expr::PseudoVar(hir::PseudoVar::False) => e.1 = hir::Ty::Bool,
            hir::Expr::PseudoVar(hir::PseudoVar::Null) => e.1 = hir::Ty::Null,
            hir::Expr::LVarRef(name) => {
                if let Some(ty) = lvars.get(name) {
                    e.1 = ty.clone();
                } else {
                    return Err(anyhow!("[BUG] unknown variable `{name}'"));
                }
            }
            hir::Expr::ArgRef(i) => e.1 = self.current_func_params.unwrap()[*i].ty.clone(),
            hir::Expr::FuncRef(name) => {
                if let Some(fun_ty) = self.sigs.get(name) {
                    e.1 = hir::Ty::Fun(fun_ty.clone());
                } else {
                    return Err(anyhow!("[BUG] unknown function `{name}'"));
                }
            }
            hir::Expr::OpCall(op, l, r) => {
                self.compile_expr(lvars, &mut *l)?;
                self.compile_expr(lvars, &mut *r)?;
                e.1 = match &op[..] {
                    "+" | "-" | "*" | "/" => hir::Ty::Int,
                    "<" | "<=" | ">" | ">=" | "==" | "!=" => hir::Ty::Bool,
                    _ => return Err(anyhow!("[BUG] unknown operator: {op}")),
                };
            }
            hir::Expr::FunCall(fexpr, arg_exprs) => {
                self.compile_expr(lvars, &mut *fexpr)?;
                let hir::Ty::Fun(fun_ty) = &fexpr.1 else {
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
                for e in arg_exprs {
                    self.compile_expr(lvars, e)?;
                }
                e.1 = *fun_ty.ret_ty.clone();
            }
            hir::Expr::If(cond, then, els) => {
                self.compile_expr(lvars, &mut *cond)?;
                self.compile_exprs(lvars, then)?;
                self.compile_exprs(lvars, els)?;
                e.1 = hir::Ty::Void;
            }
            //hir::Expr::ValuedIf(cond, then, els) => {
            //    let cond = self.compile_expr(lvars, cond)?;
            //    if cond.1 != hir::Ty::Bool {
            //        return Err(anyhow!("condition should be bool but got {:?}", cond.1));
            //    }
            //    let then = self.compile_expr(lvars, then)?;
            //    let els = self.compile_expr(lvars, els)?;
            //    if then.1 != els.1 {
            //        return Err(anyhow!(
            //            "then and else should have the same type but got {:?} and {:?}",
            //            then.1,
            //            els.1
            //        ));
            //    }
            //    e.1 = then.1.clone();
            //}
            //hir::Expr::Yield(val) => {
            //    self.compile_expr(lvars, val)?;
            //    e.1 = hir::Ty::Void;
            //}
            hir::Expr::While(cond, body) => {
                self.compile_expr(lvars, cond)?;
                self.compile_exprs(lvars, body)?;
                e.1 = hir::Ty::Void;
            }
            hir::Expr::Alloc(name) => {
                // Milika vars are always Int now
                lvars.insert(name.clone(), hir::Ty::Int);
                e.1 = hir::Ty::Void;
            }
            hir::Expr::Assign(_, val) => {
                self.compile_expr(lvars, val)?;
                e.1 = hir::Ty::Void;
            }
            hir::Expr::Return(val) => {
                self.compile_expr(lvars, val)?;
                if val.1 != *self.current_func_ret_ty.unwrap() {
                    return Err(anyhow!(
                        "return type mismatch: {} should return {:?} but got {:?}",
                        self.current_func_name.unwrap(),
                        self.current_func_ret_ty.unwrap(),
                        val.1
                    ));
                }
                e.1 = hir::Ty::Void;
            }
            hir::Expr::Cast(_, _) => {
                return Err(anyhow!("[BUG] Cast unexpected here"));
            }
        };
        Ok(())
    }

    fn compile_exprs(
        &mut self,
        lvars: &mut HashMap<String, hir::Ty>,
        es: &mut [hir::TypedExpr],
    ) -> Result<()> {
        es.iter_mut()
            .try_for_each(|e| self.compile_expr(lvars, e))?;
        Ok(())
    }
}
