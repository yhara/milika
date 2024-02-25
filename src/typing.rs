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
            ast::Declaration::Extern((e, _)) => externs.push(e.into()),
            ast::Declaration::Function((f, _)) => {
                let mut split_funcs = c.compile_func(f)?;
                funcs.append(&mut split_funcs);
            }
        }
    }
    Ok(hir::Program { externs, funcs })
}

impl Typing {
    fn compile_func(&self, f: ast::Function) -> Result<Vec<hir::Function>> {
        let body_stmts = f
            .body_stmts
            .into_iter()
            .map(|e| self.compile_expr(&f.params, e.0))
            .collect::<Result<Vec<_>>>()?;
        Ok(hir::Function {
            name: f.name,
            params: f.params.into_iter().map(|x| x.into()).collect::<Vec<_>>(),
            ret_ty: f.ty.into(),
            body_stmts,
        })
    }

    fn compile_expr(&self, params: &[ast::Param], e: ast::Expr) -> Result<(hir::Expr, hir::Ty)> {
        let hir_expr = match e {
            ast::Expr::Number(n) => {
                let ty = hir::Ty::Raw("int".to_string());
                (hir::Expr::Number(n), ty)
            }
            ast::Expr::VarRef(name) => {
                let ty = if let Some(fun_ty) = self.sigs.get(name) {
                    fun_ty
                } else if let Some(p) = self.params.iter().find(|x| x.name == name) {
                    p.ty.clone()
                } else {
                    return Err(anyhow!("unknown variable `{name}'"));
                };
                (hir::Expr::VarRef(name), ty)
            }
            ast::Expr::OpCall(op, lhs, rhs) => {
                let ty = match *op[..] {
                    "+" | "-" | "*" | "/" => hir::Ty::Int,
                    "==" | "!=" | "<" | "<=" | ">" | ">=" => hir::Ty::Bool,
                };
                let l = self.compile_expr(params, lhs.0)?;
                let r = self.compile_expr(params, lhs.0)?;
                (hir::Expr::OpCall(op, Box::new(l), Box::new(r)), ty)
            }
            ast::Expr::FunCall(fexpr, arg_exprs) => {
                let f = self.compile_expr(params, fexpr.0)?;
                let args = arg_exprs
                    .into_iter()
                    .map(|e| self.compile_expr(params, e.0))
                    .collect::<Result<Vec<_>>>()?;
                let ty = todo!();
                (hir::Expr::FunCall(Box::new(f), args), ty)
            }
            ast::Expr::If(cond_expr, then_exprs, else_exprs) => {
                let cond = self.compile_expr(params, cond_expr.0)?;
                if cond.1 != hir::Ty::Bool {
                    return Err(anyhow!("if condition must be Bool"));
                }
                let then = self.compile_exprs(params, then_exprs)?;
                let els = self.compile_exprs(params, else_exprs)?;
                let ty = hir::Ty::Void;
                (hir::Expr::If(Box::new(cond), then, els), ty)
            }
            ast::Expr::While(cond_expr, body_exprs) => {
                let cond = self.compile_expr(params, cond_expr.0)?;
                if cond.1 != hir::Ty::Bool {
                    return Err(anyhow!("while condition must be Bool"));
                }
                let body = self.compile_exprs(params, body_exprs)?;
                let ty = hir::Ty::Void;
                (hir::Expr::While(Box::new(cond), body), ty)
            }
            ast::Expr::Alloc(name) => {
                let ty = hir::Ty::Void;
                (hir::Expr::Alloc(name), ty)
            }
            (ast::Expr::Assign(name, rhs), pos) => {
                let r = self.compile_expr(params, rhs.0)?;
                let ty = hir::Ty::Void;
                (hir::Expr::Assign(name, Box::new(r)), ty)
            }
            (ast::Expr::Return(val_expr), pos) => {
                let v = self.compile_expr(params, val_expr.0)?;
                let ty = hir::Ty::Void;
                (hir::Expr::Assign(name, Box::new(v)), ty)
            }
            _ => todo!("{:?}", s_expr),
        };
        Ok(hir_expr)
    }

    fn compile_exprs(
        &self,
        params: &[ast::Param],
        es: Vec<ast::Expr>,
    ) -> Result<Vec<(hir::Expr, hir::Ty)>> {
        es.into_iter()
            .map(|e| self.compile_expr(params, e.0))
            .collect()
    }
}
