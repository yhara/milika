use crate::hir;
use anyhow::{Context, Result};

pub trait HirRewriter {
    /// Callback function.
    fn rewrite_expr(&mut self, expr: hir::TypedExpr) -> Result<hir::TypedExpr>;

    fn walk_hir(&mut self, hir: hir::Program) -> Result<hir::Program> {
        let funcs = hir.funcs.into_iter().map(|f| {
            let new_body_stmts = self.walk_exprs(&f.body_stmts)?;
            Ok(hir::Func {
                body_stmts: new_body_stmts,
                ..f
            })
        }).collect::<Result<Vec<_>>>()?;
        Ok(hir::Program {
            externs: hir.externs,
            funcs,
        })
    }

    fn walk_exprs(&mut self, exprs: Vec<hir::TypedExpr>) -> Result<Vec<hir::TypedExpr>> {
        exprs.into_iter().map(|expr| self.rewrite_expr(expr)).collect()
    }

    fn walk_expr(&mut self, expr: hir::TypedExpr) -> Result<hir::TypedExpr> {
        let new_expr = match expr.0 {
            hir::Expr::Number(_) => {}
            hir::Expr::PseudoVar(_) => {}
            hir::Expr::LVarRef(_) => {}
            hir::Expr::ArgRef(_) => {}
            hir::Expr::FuncRef(_) => {}
            hir::Expr::OpCall(_, lhs, rhs) => {
                let new_lhs = self.rewrite_expr(lhs)?;
                let new_rhs = self.rewrite_expr(rhs)?;
                hir::Expr::OpCall(new_lhs, new_rhs)
            }
            hir::Expr::FunCall(fexpr, arg_exprs) => {
                let new_fexpr = self.rewrite_expr(fexpr)?;
                let new_arg_exprs = self.rewrite_exprs(arg_exprs)?;
                hir::Expr::FunCall(new_fexpr, new_arg_exprs)
            }
            hir::Expr::If(cond_expr, then_exprs, else_exprs) => {
                let new_cond_expr = self.rewrite_expr(cond_expr)?;
                let new_then_exprs = self.rewrite_exprs(then_exprs)?;
                let new_else_exprs = self.rewrite_exprs(else_exprs)?;
                hir::Expr::If(new_cond_expr, new_then_exprs, new_else_exprs)
            }
            hir::Expr::ValuedIf(cond_expr, then_exprs, else_exprs) => {
                let new_cond_expr = self.rewrite_expr(cond_expr)?;
                let new_then_exprs = self.rewrite_exprs(then_exprs)?;
                let new_else_exprs = self.rewrite_exprs(else_exprs)?;
                hir::Expr::ValuedIf(new_cond_expr, new_then_exprs, new_else_exprs)
            }
            hir::Expr::Yield(expr) => {
                let new_expr = self.rewrite_expr(expr)?;
                hir::Expr::Yield(new_expr)
            }
            hir::Expr::While(cond_expr, body_exprs) => {
                let new_cond_expr = self.rewrite_expr(cond_expr)?;
                let new_body_exprs = self.rewrite_exprs(body_exprs)?;
                hir::Expr::While(new_cond_expr, new_body_exprs)
            }
            hir::Expr::Alloc(_) => {}
            hir::Expr::Assign(name, rhs) => {
                let new_rhs = self.rewrite_expr(rhs)?;
                hir::Expr::Assign(name, new_rhs)
            }
            hir::Expr::Return(expr) => {
                let new_expr = self.rewrite_expr(expr)?;
                hir::Expr::Return(new_expr)
            }
            hir::Expr::Cast(cast_type, expr) => {
                let new_expr = self.rewrite_expr(expr)?;
                hir::Expr::Cast(cast_type, new_expr)
            }
        };
        Ok(self.rewrite_expr(hir::TypedExpr(new_expr, expr.1))?)
    }
}
