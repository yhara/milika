use crate::hir;
use anyhow::Result;

pub trait HirRewriter {
    /// Callback function.
    fn rewrite_expr(&mut self, expr: hir::TypedExpr) -> Result<hir::TypedExpr>;

    fn walk_hir(&mut self, hir: hir::Program) -> Result<hir::Program> {
        let funcs = hir
            .funcs
            .into_iter()
            .map(|f| {
                let body_stmts = self.walk_exprs(f.body_stmts)?;
                Ok(hir::Function { body_stmts, ..f })
            })
            .collect::<Result<_>>()?;
        Ok(hir::Program { funcs, ..hir })
    }

    fn walk_shir(&mut self, shir: hir::split::Program) -> Result<hir::split::Program> {
        let mut funcs = vec![];
        for group in shir.funcs {
            let mut new_group = vec![];
            for f in group {
                let body_stmts = self.walk_exprs(f.body_stmts)?;
                new_group.push(hir::Function { body_stmts, ..f });
            }
            funcs.push(new_group);
        }
        Ok(hir::split::Program { funcs, ..shir })
    }

    fn walk_exprs(&mut self, exprs: Vec<hir::TypedExpr>) -> Result<Vec<hir::TypedExpr>> {
        exprs.into_iter().map(|expr| self.walk_expr(expr)).collect()
    }

    fn walk_expr(&mut self, expr: hir::TypedExpr) -> Result<hir::TypedExpr> {
        let new_expr = match expr.0 {
            hir::Expr::Number(_) => expr,
            hir::Expr::PseudoVar(_) => expr,
            hir::Expr::LVarRef(_) => expr,
            hir::Expr::ArgRef(_) => expr,
            hir::Expr::FuncRef(_) => expr,
            hir::Expr::OpCall(op, lhs, rhs) => {
                hir::Expr::op_call(op, self.walk_expr(*lhs)?, self.walk_expr(*rhs)?)
            }
            hir::Expr::FunCall(fexpr, arg_exprs) => {
                hir::Expr::fun_call(self.walk_expr(*fexpr)?, self.walk_exprs(arg_exprs)?)
            }
            hir::Expr::If(cond_expr, then_exprs, else_exprs) => hir::Expr::if_(
                self.walk_expr(*cond_expr)?,
                self.walk_exprs(then_exprs)?,
                self.walk_exprs(else_exprs)?,
            ),
            hir::Expr::Yield(expr) => hir::Expr::yield_(self.walk_expr(*expr)?),
            hir::Expr::While(cond_expr, body_exprs) => {
                hir::Expr::while_(self.walk_expr(*cond_expr)?, self.walk_exprs(body_exprs)?)
            }
            hir::Expr::Alloc(_) => expr,
            hir::Expr::Assign(name, rhs) => hir::Expr::assign(name, self.walk_expr(*rhs)?),
            hir::Expr::Return(expr) => hir::Expr::return_(self.walk_expr(*expr)?),
            hir::Expr::Cast(cast_type, expr) => hir::Expr::cast(cast_type, self.walk_expr(*expr)?),
            hir::Expr::CondReturn(cond, fexpr_t, args_t, fexpr_f, args_f) => {
                hir::Expr::cond_return(
                    self.walk_expr(*cond)?,
                    self.walk_expr(*fexpr_t)?,
                    self.walk_exprs(args_t)?,
                    self.walk_expr(*fexpr_f)?,
                    self.walk_exprs(args_f)?,
                )
            }
            _ => panic!("not supported by hir::rewriter: {:?}", expr),
        };
        self.rewrite_expr(new_expr)
    }
}
