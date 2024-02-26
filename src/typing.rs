use crate::ast;
use crate::asyncness_check;
use crate::hir;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

struct Typing {
    sigs: HashMap<String, hir::FunTy>,
}

/// Converts ast to hir. Also does some typecheck
pub fn run(ast: ast::Program) -> Result<hir::Program> {
    let c = Typing {
        sigs: asyncness_check::gather_sigs(&ast.0)?,
    };
    let mut externs = vec![];
    let mut funcs = vec![];
    for decl in ast.0 {
        match decl {
            ast::Declaration::Extern((e, _)) => externs.push(e.try_into()?),
            ast::Declaration::Function((f, _)) => {
                funcs.push(c.compile_func(f)?);
            }
        }
    }
    Ok(hir::Program { externs, funcs })
}

impl Typing {
    fn compile_func(&self, f: ast::Function) -> Result<hir::Function> {
        let body_stmts = f
            .body_stmts
            .iter()
            .map(|e| self.compile_expr(&f, &e.0))
            .collect::<Result<Vec<_>>>()?;
        Ok(hir::Function {
            name: f.name,
            params: f
                .params
                .into_iter()
                .map(|x| x.try_into())
                .collect::<Result<Vec<_>>>()?,
            ret_ty: f.ret_ty.try_into()?,
            body_stmts,
        })
    }

    fn compile_expr(
        &self,
        orig_func: &ast::Function,
        e: &ast::Expr,
    ) -> Result<(hir::Expr, hir::Ty)> {
        let hir_expr = match e {
            ast::Expr::Number(n) => {
                let ty = hir::Ty::Int;
                (hir::Expr::Number(*n), ty)
            }
            ast::Expr::VarRef(name) => {
                let ty = if let Some(fun_ty) = self.sigs.get(name) {
                    fun_ty.clone().into()
                } else if let Some(p) = orig_func.params.iter().find(|x| &x.name == name) {
                    p.ty.clone().try_into()?
                } else {
                    return Err(anyhow!("unknown variable `{name}'"));
                };
                (hir::Expr::VarRef(name.to_string()), ty)
            }
            ast::Expr::OpCall(op, lhs, rhs) => {
                let ty = match &op[..] {
                    "+" | "-" | "*" | "/" => hir::Ty::Int,
                    "==" | "!=" | "<" | "<=" | ">" | ">=" => hir::Ty::Bool,
                    _ => return Err(anyhow!("[BUG] unknown operator `{op}'")),
                };
                let l = self.compile_expr(orig_func, &lhs.0)?;
                let r = self.compile_expr(orig_func, &rhs.0)?;
                (
                    hir::Expr::OpCall(op.to_string(), Box::new(l), Box::new(r)),
                    ty,
                )
            }
            ast::Expr::FunCall(fexpr, arg_exprs) => {
                let f = self.compile_expr(orig_func, &fexpr.0)?;
                let hir::Ty::Fun(fun_ty) = &f.1 else {
                    return Err(anyhow!("not a function: {:?}", f.1));
                };
                if fun_ty.param_tys.len() != arg_exprs.len() {
                    return Err(anyhow!(
                        "funcall arity mismatch (expected {}, got {}): {:?}",
                        fun_ty.param_tys.len(),
                        arg_exprs.len(),
                        e
                    ));
                }
                let args = arg_exprs
                    .into_iter()
                    .map(|e| self.compile_expr(orig_func, &e.0))
                    .collect::<Result<Vec<_>>>()?;
                check_funcall_arg_types(&fun_ty.param_tys, &args)?;
                let ty = (*fun_ty.ret_ty).clone();
                (hir::Expr::FunCall(Box::new(f), args), ty)
            }
            ast::Expr::If(cond_expr, then_exprs, opt_else_exprs) => {
                let cond = self.compile_expr(orig_func, &cond_expr.0)?;
                if cond.1 != hir::Ty::Bool {
                    return Err(anyhow!("if condition must be Bool"));
                }
                let then = self.compile_exprs(orig_func, then_exprs)?;
                let els = if let Some(es) = opt_else_exprs {
                    Some(self.compile_exprs(orig_func, es)?)
                } else {
                    None
                };
                let ty = hir::Ty::Void;
                (hir::Expr::If(Box::new(cond), then, els), ty)
            }
            ast::Expr::While(cond_expr, body_exprs) => {
                let cond = self.compile_expr(orig_func, &cond_expr.0)?;
                if cond.1 != hir::Ty::Bool {
                    return Err(anyhow!("while condition must be Bool"));
                }
                let body = self.compile_exprs(orig_func, body_exprs)?;
                let ty = hir::Ty::Void;
                (hir::Expr::While(Box::new(cond), body), ty)
            }
            ast::Expr::Alloc(name) => {
                let ty = hir::Ty::Void;
                (hir::Expr::Alloc(name.to_string()), ty)
            }
            ast::Expr::Assign(name, rhs) => {
                let r = self.compile_expr(orig_func, &rhs.0)?;
                let ty = hir::Ty::Void;
                (hir::Expr::Assign(name.to_string(), Box::new(r)), ty)
            }
            ast::Expr::Return(val_expr) => {
                let v = self.compile_expr(orig_func, &val_expr.0)?;
                if v.1 != orig_func.ret_ty.clone().try_into()? {
                    return Err(anyhow!("return type mismatch"));
                }
                let ty = hir::Ty::Void;
                (hir::Expr::Return(Box::new(v)), ty)
            }
            _ => todo!("{:?}", e),
        };
        Ok(hir_expr)
    }

    fn compile_exprs(
        &self,
        orig_func: &ast::Function,
        es: &[ast::SpannedExpr],
    ) -> Result<Vec<(hir::Expr, hir::Ty)>> {
        es.iter()
            .map(|e| self.compile_expr(orig_func, &e.0))
            .collect()
    }
}

fn check_funcall_arg_types(param_tys: &[hir::Ty], args: &[(hir::Expr, hir::Ty)]) -> Result<()> {
    for (param_ty, (_, arg_ty)) in param_tys.iter().zip(args.iter()) {
        if param_ty != arg_ty {
            return Err(anyhow!(
                "funcall arg type mismatch: expected {:?} but got {:?}",
                param_ty,
                arg_ty
            ));
        }
    }
    Ok(())
}
