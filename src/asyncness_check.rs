use crate::ast::{self, FunTy};
use anyhow::{anyhow, Result};
use either::Either;
use std::collections::HashMap;

type FuncName = String;

pub fn gather_sigs(decls: &[ast::Declaration]) -> Result<HashMap<String, ast::FunTy>> {
    let mut sigs = HashMap::new();
    let mut funcs = HashMap::new();
    let mut queue = vec![];
    // 1st pass
    for decl in decls {
        match decl {
            ast::Declaration::Extern((x, _span)) => {
                sigs.insert(x.name.clone(), x.fun_ty());
            }
            ast::Declaration::Function((x, _span)) => {
                funcs.insert(x.name.clone(), x);
                queue.push(x.name.clone());
            }
        }
    }

    // 2nd pass
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

fn gather_sig(
    func: &ast::Function,
    sigs: &HashMap<String, FunTy>,
) -> Result<Either<FuncName, FunTy>> {
    let mut is_async = false;
    for (stmt, _) in &func.body_stmts {
        match check_async(&func.name, &stmt, sigs)? {
            Either::Left(x) => return Ok(Either::Left(x)),
            Either::Right(b) => is_async = is_async || b,
        }
    }
    Ok(Either::Right(func.fun_ty(is_async)))
}

fn check_async(
    func_name: &str,
    expr: &ast::Expr,
    sigs: &HashMap<String, FunTy>,
) -> Result<Either<FuncName, bool>> {
    match expr {
        ast::Expr::FunCall(fexpr, arg_exprs) => {
            let (ast::Expr::VarRef(ref fname), _) = **fexpr else {
                return Err(anyhow!("not a function: {:?}", fexpr));
            };
            if let Some(fun_ty) = sigs.get(fname) {
                if fun_ty.is_async {
                    Ok(Either::Right(true))
                } else {
                    let mut is_async = false;
                    for (e, _) in arg_exprs {
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
