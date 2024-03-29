use crate::ast::{self};
use crate::hir;
use anyhow::{anyhow, Result};
use either::Either;
use std::collections::HashMap;

type FuncName = String;

/// Gather function signatures from declarations and change
/// return types of async functions to Async(_)
pub fn gather_sigs(decls: &[ast::Declaration]) -> Result<HashMap<String, hir::FunTy>> {
    let mut sigs = HashMap::new();
    let mut funcs = HashMap::new();
    let mut queue = vec![];
    // 1st pass
    // Check asyncness of externs
    // Collect function names
    for decl in decls {
        match decl {
            ast::Declaration::Extern(x) => {
                sigs.insert(x.name.clone(), hir::Extern::from_ast(x)?.fun_ty());
            }
            ast::Declaration::Function(x) => {
                funcs.insert(x.name.clone(), x);
                queue.push(x.name.clone());
            }
        }
    }

    // 2nd pass
    // Check asyncness of functions until all are resolved
    while let Some(func_name) = queue.pop() {
        let func = funcs.get(&func_name).unwrap();
        match gather_sig(func, &sigs)? {
            Either::Left(called_fname) => {
                if !queue.contains(&called_fname) {
                    queue.push(called_fname);
                }
            }
            Either::Right(sig) => {
                sigs.insert(func.name.clone(), sig);
            }
        }
    }

    Ok(sigs)
}

/// Check asyncness of a function
/// Returns the function name if it depends on the asyncness of another function
fn gather_sig(
    func: &ast::Function,
    sigs: &HashMap<String, hir::FunTy>,
) -> Result<Either<FuncName, hir::FunTy>> {
    let mut is_async = false;
    for stmt in &func.body_stmts {
        match check_async(&func.name, &stmt, sigs)? {
            Either::Left(x) => return Ok(Either::Left(x)),
            Either::Right(b) => is_async = is_async || b,
        }
    }
    dbg!(&func.name, is_async);
    Ok(Either::Right(hir::FunTy::from_ast_func(func, is_async)?))
}

/// Check if the expression is async
/// Returns the function name if it depends on the asyncness of another function
fn check_async(
    func_name: &str,
    expr: &ast::Expr,
    sigs: &HashMap<String, hir::FunTy>,
) -> Result<Either<FuncName, bool>> {
    match expr {
        ast::Expr::FunCall(fexpr, arg_exprs) => {
            let ast::Expr::VarRef(ref fname) = **fexpr else {
                return Err(anyhow!("not a function: {:?}", fexpr));
            };
            if let Some(fun_ty) = sigs.get(fname) {
                dbg!(func_name, fname, fun_ty);
                if fun_ty.ret_ty.is_async() {
                    // This function has an async call.
                    Ok(Either::Right(true))
                } else {
                    let mut is_async = false;
                    for e in arg_exprs {
                        match check_async(func_name, e, sigs)? {
                            Either::Left(x) => return Ok(Either::Left(x)),
                            Either::Right(b) => is_async = is_async || b,
                        }
                    }
                    Ok(Either::Right(is_async))
                }
            } else if fname == func_name {
                // Calling the function itself
                Ok(Either::Right(false))
            } else {
                // Depends on the asyncness of this function
                Ok(Either::Left(fname.to_string()))
            }
        }
        _ => Ok(Either::Right(false)),
    }
}

pub struct CheckAsync(());
impl CheckAsync {
    fn check() -> Result<bool, FuncName> {
    }
}
impl HirVisitor for CheckAsync {
    fn visit_expr(&mut self, texpr: &hir::TypedExpr) -> Result<bool, FuncName> {

        assert_no_async_ty(&texpr.1).context(format!("in expr: {:?}", texpr))
    }
}

