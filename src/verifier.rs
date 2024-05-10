use crate::hir;
use anyhow::{bail, Context, Result};

/// Check type consistency of the HIR to detect bugs in the compiler.
pub fn run(hir: &hir::blocked::Program) -> Result<()> {
    for f in &hir.funcs {
        for b in &f.body_blocks {
            for e in &b.stmts {
                verify_expr(f, e)?;
            }
        }
    }
    Ok(())
}

fn verify_expr(f: &hir::blocked::Function, e: &hir::TypedExpr) -> Result<()> {
    _verify_expr(f, e)
        .context(format!("in expr {:?}", e.0))
        .context(format!("in function {:?}", f.name))
        .context(format!("[BUG] Type verifier failed"))
}

fn _verify_expr(f: &hir::blocked::Function, e: &hir::TypedExpr) -> Result<()> {
    match &e.0 {
        hir::Expr::Number(_) => assert(&e, "number", &hir::Ty::Int)?,
        hir::Expr::PseudoVar(_) => (),
        hir::Expr::LVarRef(_) => (),
        hir::Expr::ArgRef(_) => (),
        hir::Expr::FuncRef(_) => (),
        hir::Expr::OpCall(_, a, b) => {
            verify_expr(f, a)?;
            verify_expr(f, b)?;
        }
        hir::Expr::FunCall(fexpr, args) => {
            verify_expr(f, fexpr)?;
            for a in args {
                verify_expr(f, a)?;
            }
            let hir::Ty::Fun(fun_ty) = &fexpr.1 else {
                bail!("expected function, but got {:?}", fexpr.1);
            };
            fun_ty
                .param_tys
                .iter()
                .enumerate()
                .zip(args.iter())
                .try_for_each(|((i, p), a)| assert(&a, &format!("argument {}", i), p))?;
        }
        hir::Expr::If(cond, then, els) => {
            verify_expr(f, cond)?;
            verify_exprs(f, then)?;
            verify_exprs(f, els)?;
        }
        hir::Expr::Yield(expr) => {
            verify_expr(f, expr)?;
        }
        hir::Expr::While(cond, body) => {
            verify_expr(f, cond)?;
            verify_exprs(f, body)?;
        }
        hir::Expr::Alloc(_) => (),
        hir::Expr::Assign(_, v) => {
            verify_expr(f, v)?;
        }
        hir::Expr::Return(e) => {
            verify_expr(f, e)?;
            assert(&e, "return value", &f.ret_ty)?;
        }
        hir::Expr::Cast(cast_type, val) => {
            verify_expr(f, val)?;
            match cast_type {
                hir::CastType::AnyToFun(fun_ty) => {
                    assert(&val, "castee", &hir::Ty::Any)?;
                    assert(&e, "result", &fun_ty.clone().into())?;
                }
                hir::CastType::AnyToInt => {
                    assert(&val, "castee", &hir::Ty::Any)?;
                    assert(&e, "result", &hir::Ty::Int)?;
                }
                hir::CastType::IntToAny => {
                    assert(&val, "castee", &hir::Ty::Int)?;
                    assert(&e, "result", &hir::Ty::Any)?;
                }
                hir::CastType::FunToAny => {
                    assert_fun(&val.1)?;
                    assert(&e, "result", &hir::Ty::Any)?;
                }
            }
        }
        hir::Expr::CondReturn(cond, fexpr_t, args_t, fexpr_f, args_f) => {
            verify_expr(f, cond)?;
            verify_expr(f, fexpr_t)?;
            verify_exprs(f, args_t)?;
            verify_expr(f, fexpr_f)?;
            verify_exprs(f, args_f)?;
        }
        hir::Expr::Br(e, n) => {
            if *n >= f.body_blocks.len() {
                bail!("block index out of range: {}", n);
            }
            verify_expr(f, e)?;
        }
        hir::Expr::CondBr(cond, then, els) => {
            verify_expr(f, cond)?;
            if *then >= f.body_blocks.len() {
                bail!("block index out of range: {}", then);
            }
            if *els >= f.body_blocks.len() {
                bail!("block index out of range: {}", els);
            }
        }
        hir::Expr::BlockArgRef => (),
    }
    Ok(())
}

fn verify_exprs(f: &hir::blocked::Function, es: &[hir::TypedExpr]) -> Result<()> {
    for e in es {
        verify_expr(f, e)?;
    }
    Ok(())
}

fn assert(v: &hir::TypedExpr, for_: &str, expected: &hir::Ty) -> Result<()> {
    if v.1 != *expected {
        bail!("expected {:?} for {for_}, but got {:?}", expected, v);
    }
    Ok(())
}

fn assert_fun(ty: &hir::Ty) -> Result<()> {
    if !matches!(ty, hir::Ty::Fun(_)) {
        bail!("expected Ty::Fun, but got {:?}", ty);
    }
    Ok(())
}
