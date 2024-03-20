use crate::hir;
use anyhow::{bail, Context, Result};

/// Check type consistency of the HIR to detect bugs in the compiler.
pub fn run(hir: &hir::Program) -> Result<()> {
    for f in &hir.funcs {
        for e in &f.body_stmts {
            verify_expr(f, e)?;
        }
    }
    Ok(())
}

fn verify_expr(f: &hir::Function, e: &hir::TypedExpr) -> Result<()> {
    _verify_expr(f, e)
        .context(format!("in expr {:?}", e.0))
        .context(format!("in function {:?}", f.name))
        .context(format!("[BUG] Type verifier failed"))
}

fn _verify_expr(f: &hir::Function, e: &hir::TypedExpr) -> Result<()> {
    match &e.0 {
        hir::Expr::Number(_) => assert(&e.1, &hir::Ty::Int)?,
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
                .zip(args.iter())
                .try_for_each(|(p, a)| assert(&a.1, p))?;
        }
        hir::Expr::If(cond, then, els) => {
            verify_expr(f, cond)?;
            verify_exprs(f, then)?;
            if let Some(el) = els {
                verify_exprs(f, el)?;
            }
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
            assert(&e.1, &f.ret_ty)?;
        }
        hir::Expr::Cast(cast_type, val) => {
            verify_expr(f, val)?;
            match cast_type {
                hir::CastType::AnyToFun(fun_ty) => {
                    assert(&val.1, &hir::Ty::Any)?;
                    assert(&e.1, &fun_ty.clone().into())?;
                }
                hir::CastType::AnyToInt => {
                    assert(&val.1, &hir::Ty::Any)?;
                    assert(&e.1, &hir::Ty::Int)?;
                }
                hir::CastType::IntToAny => {
                    assert(&val.1, &hir::Ty::Int)?;
                    assert(&e.1, &hir::Ty::Any)?;
                }
                hir::CastType::FunToAny => {
                    assert_fun(&val.1)?;
                    assert(&e.1, &hir::Ty::Any)?;
                }
            }
        }
        hir::Expr::Para(es) => {
            verify_exprs(f, es)?;
        }
    }
    Ok(())
}

fn verify_exprs(f: &hir::Function, es: &[hir::TypedExpr]) -> Result<()> {
    for e in es {
        verify_expr(f, e)?;
    }
    Ok(())
}

fn assert(ty: &hir::Ty, expected: &hir::Ty) -> Result<()> {
    if ty != expected {
        bail!("expected {:?}, but got {:?}", expected, ty);
    }
    Ok(())
}

fn assert_fun(ty: &hir::Ty) -> Result<()> {
    if !matches!(ty, hir::Ty::Fun(_)) {
        bail!("expected Ty::Fun, but got {:?}", ty);
    }
    Ok(())
}
