use crate::hir;
use anyhow::Result;

pub trait HirVisitor {
    /// Callback function.
    fn visit_expr(&mut self, expr: &hir::TypedExpr) -> Result<()>;

    fn walk_hir(&mut self, hir: &hir::Program) -> Result<()> {
        for f in &hir.funcs {
            self.walk_exprs(&f.body_stmts)?;
        }
        Ok(())
    }

    fn walk_exprs(&mut self, exprs: &[hir::TypedExpr]) -> Result<()> {
        for expr in exprs {
            self.walk_expr(expr)?;
        }
        Ok(())
    }

    fn walk_expr(&mut self, expr: &hir::TypedExpr) -> Result<()> {
        match &expr.0 {
            hir::Expr::Number(_) => {}
            hir::Expr::PseudoVar(_) => {}
            hir::Expr::LVarRef(_) => {}
            hir::Expr::ArgRef(_) => {}
            hir::Expr::FuncRef(_) => {}
            hir::Expr::OpCall(_, lhs, rhs) => {
                self.visit_expr(lhs)?;
                self.visit_expr(rhs)?;
            }
            hir::Expr::FunCall(fexpr, arg_exprs) => {
                self.visit_expr(fexpr)?;
                for arg in arg_exprs {
                    self.visit_expr(arg)?;
                }
            }
            hir::Expr::If(cond_expr, then_exprs, else_exprs) => {
                self.visit_expr(cond_expr)?;
                for expr in then_exprs {
                    self.visit_expr(expr)?;
                }
                for expr in else_exprs {
                    self.visit_expr(expr)?;
                }
            }
            hir::Expr::ValuedIf(cond_expr, then_exprs, else_exprs) => {
                self.visit_expr(cond_expr)?;
                for expr in then_exprs {
                    self.visit_expr(expr)?;
                }
                for expr in else_exprs {
                    self.visit_expr(expr)?;
                }
            }
            hir::Expr::Yield(expr) => {
                self.visit_expr(expr)?;
            }
            hir::Expr::While(cond_expr, body_exprs) => {
                self.visit_expr(cond_expr)?;
                for expr in body_exprs {
                    self.visit_expr(expr)?;
                }
            }
            hir::Expr::Alloc(_) => {}
            hir::Expr::Assign(_, rhs) => {
                self.visit_expr(rhs)?;
            }
            hir::Expr::Return(expr) => {
                self.visit_expr(expr)?;
            }
            hir::Expr::Cast(_, expr) => {
                self.visit_expr(expr)?;
            }
        }
        self.visit_expr(expr)?;
        Ok(())
    }
}

pub struct Allocs(Vec<(String, hir::Ty)>);
impl Allocs {
    pub fn collect(body_stmts: &[hir::TypedExpr]) -> Result<Vec<(String, hir::Ty)>> {
        let mut a = Allocs(vec![]);
        a.walk_exprs(body_stmts)?;
        Ok(a.0)
    }
}
impl HirVisitor for Allocs {
    fn visit_expr(&mut self, texpr: &hir::TypedExpr) -> Result<()> {
        match texpr {
            (hir::Expr::Alloc(name), ty) => {
                self.0.push((name.clone(), ty.clone()));
            }
            _ => {}
        }
        Ok(())
    }
}
