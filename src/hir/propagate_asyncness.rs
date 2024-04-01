use crate::hir;
use std::collections::VecDeque;
use std::collections::HashSet;

pub fn run(hir: mut hir::Program) -> hir::Program {
    let mut func_asyncness = HashMap::new();
    for e in hir.externs {
        func_asyncness.insert(e.name.clone(), e.ret_ty.is_async());
    }

    let mut queue = VecDeque::new();
    for f in hir.funcs {
        queue.push_back(f.name.clone());
    }

    while let Some(func_name) = queue.pop_front() {
        match IsAsync::check(func, &sigs)? {
            Ok(b) => {
                func_asyncness.insert(func_name.clone(), b);
            }
            Err(called_fname) => {
                if !queue.contains(&called_fname) {
                    queue.push_back(called_fname);
                }
            }
        }
    }
    hir
}

pub struct IsAsync<'a> {
    func_asyncness: &'a HashMap<String, bool>,
}
impl<'a> IsAsync<'a> {
    fn check(
        func: &hir::Function,
        func_asyncness: &HashMap<String, bool>,
    ) -> Result<bool, FuncName> {
        self.func_asyncness = func_asyncness;
        self.walk_exprs(&func.body_stmts)?;
    }
}
impl HirVisitor for IsAsync {
    fn visit_expr(&mut self, texpr: &hir::TypedExpr) -> Result<bool, FuncName> {
        if let hir::Expr::FunCall(fexpr, arg_exprs) = &texpr.0 {


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
        assert_no_async_ty(&texpr.1).context(format!("in expr: {:?}", texpr))
    }
}
