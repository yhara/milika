//! Example
//! ```
//! // Before
//! fun foo() -> Int {
//!   print(sleep_sec(1));
//!   return 42;
//! }
//! // After
//! fun foo($env, $cont) -> RustFuture {
//!   chiika_env_push($env, $cont);
//!   return sleep_sec(foo_1, 1);
//! }
//! fun foo_1($env, $async_result) -> RustFuture {
//!   print($async_result);
//!   return chiika_env_pop($env)(42); // Call the original $cont
//! }
//! ```
use crate::hir;
use anyhow::{anyhow, Result};
use std::collections::VecDeque;

/// Splits asynchronous Milika func into multiple funcs.
/// Also, signatures of async externs are modified to take `$env` and `$cont` as the first two params.
pub fn run(shir: hir::split::Program) -> Result<hir::split::Program> {
    let externs = shir
        .externs
        .into_iter()
        .map(|e| {
            if e.is_async {
                hir::Extern {
                    params: append_async_params(&e.params, e.ret_ty.clone(), false),
                    ret_ty: hir::Ty::RustFuture,
                    ..e
                }
            } else {
                e
            }
        })
        .collect();

    let mut funcs = vec![];
    for group in shir.funcs {
        for mut f in group {
            let mut c = Compiler {
                orig_func: &mut f,
                chapters: Default::default(),
            };
            let split_funcs = c.compile_func()?;
            funcs.push(split_funcs);
        }
    }
    Ok(hir::split::Program::new(externs, funcs))
}

#[derive(Debug)]
struct Compiler<'a> {
    orig_func: &'a mut hir::Function,
    chapters: Chapters,
}

impl<'a> Compiler<'a> {
    /// Entry point for each milika function
    fn compile_func(&mut self) -> Result<Vec<hir::Function>> {
        self.chapters.add(Chapter::new(self.orig_func.name.clone()));
        for expr in self.orig_func.body_stmts.drain(..).collect::<Vec<_>>() {
            if let Some(new_expr) = self.compile_expr(expr, false)? {
                self.chapters.add_stmt(new_expr);
            }
        }

        let mut chaps = self.chapters.extract();
        if self.orig_func.asyncness.is_async() {
            self._generate_split_funcs(chaps)
        } else {
            // Has no async call; no modification needed
            Ok(vec![hir::Function {
                generated: self.orig_func.generated,
                asyncness: hir::Asyncness::Lowered,
                name: self.orig_func.name.clone(),
                params: self
                    .orig_func
                    .params
                    .iter()
                    .map(|x| x.clone().into())
                    .collect(),
                ret_ty: self.orig_func.ret_ty.clone().into(),
                body_stmts: chaps.pop_front().unwrap().stmts,
            }])
        }
    }

    /// Serialize `chapters` to functions
    fn _generate_split_funcs(
        &mut self,
        mut chapters: VecDeque<Chapter>,
    ) -> Result<Vec<hir::Function>> {
        let mut i = 0;
        let mut last_chap_result_ty = None;
        let mut split_funcs = vec![];
        while let Some(chap) = chapters.pop_front() {
            let new_func = if i == 0 {
                let body_stmts = if self.orig_func.asyncness.is_async() && !self.orig_func.generated
                {
                    prepend_async_intro(self.orig_func, chap.stmts)
                } else {
                    chap.stmts
                };
                // It takes `$env` and `$cont` before the original params
                let params = append_async_params(
                    &self
                        .orig_func
                        .params
                        .iter()
                        .map(|x| x.clone().into())
                        .collect::<Vec<_>>(),
                    self.orig_func.ret_ty.clone().into(),
                    self.orig_func.generated,
                );
                let current_func_arity = params.len();
                // Entry point function has the same name as the original.
                hir::Function {
                    generated: self.orig_func.generated,
                    asyncness: self.orig_func.asyncness.clone(),
                    name: self.orig_func.name.clone(),
                    params,
                    ret_ty: hir::Ty::RustFuture,
                    body_stmts: self.modify_return(current_func_arity, body_stmts),
                }
            } else {
                let params = vec![
                    hir::Param::new(hir::Ty::ChiikaEnv, "$env"),
                    // The result of the previous async call
                    hir::Param::new(last_chap_result_ty.unwrap(), "$async_result"),
                ];
                let current_func_arity = params.len();
                // The rest of the functions have a name like `foo_1`, `foo_2`, ...
                hir::Function {
                    generated: true,
                    asyncness: hir::Asyncness::Lowered,
                    name: chapter_func_name(&self.orig_func.name, i),
                    params,
                    ret_ty: hir::Ty::RustFuture,
                    body_stmts: self.modify_return(current_func_arity, chap.stmts),
                }
            };
            i += 1;
            if chapters.is_empty() {
                last_chap_result_ty = None;
            } else {
                last_chap_result_ty = Some(chap.async_result_ty.unwrap());
            }
            split_funcs.push(new_func);
        }
        Ok(split_funcs)
    }

    /// Insert call of `chiika_env_pop` before the `return`.
    fn modify_return(
        &self,
        current_func_arity: usize,
        mut body_stmts: Vec<hir::TypedExpr>,
    ) -> Vec<hir::TypedExpr> {
        if !self.orig_func.asyncness.is_async() {
            // No mod needed for a sync function
            return body_stmts;
        }
        let new_ret_value = match body_stmts.pop() {
            None => panic!("function body is empty"),
            // Jump to if-branch-functions
            Some((hir::Expr::CondReturn(cond_expr, fexpr_t, args_t, fexpr_f, args_f), _)) => {
                let call_t = modify_cond_call(*fexpr_t, args_t, &self.orig_func);
                let call_f = modify_cond_call(*fexpr_f, args_f, &self.orig_func);
                hir::Expr::if_(
                    *cond_expr,
                    vec![hir::Expr::yield_(call_t)],
                    vec![hir::Expr::yield_(call_f)],
                )
            }
            // Jump from if-branch to endif-function
            Some((hir::Expr::Branch(fname, expr), _)) => {
                let if_ty = expr.1.clone();
                let args = vec![arg_ref_env(), *expr];
                let new_fexpr = {
                    let new_fun_ty = {
                        let param_tys = vec![hir::Ty::ChiikaEnv, if_ty];
                        hir::FunTy {
                            asyncness: hir::Asyncness::Lowered,
                            param_tys,
                            ret_ty: Box::new(hir::Ty::RustFuture),
                        }
                    };
                    hir::Expr::func_ref(fname, new_fun_ty.clone())
                };
                hir::Expr::fun_call(new_fexpr, args)
            }
            Some((hir::Expr::AsyncCall(fexpr, args), _)) => {
                let new_fexpr = (fexpr.0, async_fun_ty(fexpr.1.as_fun_ty()).into());
                hir::Expr::fun_call(new_fexpr, args)
            }
            // Originated from `return` in the source text
            // Call chiika_env_pop before leaving the origin func
            Some((hir::Expr::Return(value_expr), _)) => {
                let env_pop = {
                    let n_pop = self.orig_func.params.len() + 1; // +1 for $cont
                    let cont_ty = hir::Ty::Fun(hir::FunTy {
                        asyncness: hir::Asyncness::Lowered,
                        param_tys: vec![hir::Ty::ChiikaEnv, self.orig_func.ret_ty.clone()],
                        ret_ty: Box::new(hir::Ty::RustFuture),
                    });
                    call_chiika_env_pop(n_pop, cont_ty, current_func_arity)
                };
                if value_expr.0.is_async_fun_call() {
                    // Convert `callee(args...)`
                    // to `callee(args..., env, env_pop())`
                    let hir::Expr::FunCall(fexpr, mut args) = value_expr.0 else {
                        unreachable!();
                    };
                    args.insert(0, arg_ref_env());
                    args.push(env_pop);
                    let new_fexpr = (fexpr.0, async_fun_ty(fexpr.1.as_fun_ty()).into());
                    hir::Expr::fun_call(new_fexpr, args)
                } else {
                    // `(env_pop())(env, value)`
                    hir::Expr::fun_call(env_pop, vec![arg_ref_env(), *value_expr])
                }
            }
            x => panic!("[BUG] unexpected last stmt: {:?}", x),
        };
        // Call chiika_env_pop before leaving the origin func
        body_stmts.push(hir::Expr::return_(new_ret_value));
        body_stmts
    }

    fn compile_value_expr(&mut self, e: hir::TypedExpr, on_return: bool) -> Result<hir::TypedExpr> {
        if let Some(expr) = self.compile_expr(e, on_return)? {
            Ok(expr)
        } else {
            Err(anyhow!("[BUG] unexpected void expr"))
        }
    }

    /// Examine each expression for special care. Most important part is
    /// calling async function.
    fn compile_expr(
        &mut self,
        e: hir::TypedExpr,
        on_return: bool,
    ) -> Result<Option<hir::TypedExpr>> {
        let new_e = match e.0 {
            hir::Expr::Number(_) => e,
            hir::Expr::PseudoVar(_) => e,
            hir::Expr::LVarRef(_) => {
                if self.chapters.len() == 1 {
                    // The variable is just there in the first chapter
                    e
                } else {
                    // We need to carry the variable via env
                    todo!()
                }
            }
            hir::Expr::ArgRef(idx) => {
                if self.chapters.len() == 1 {
                    hir::Expr::arg_ref(idx, e.1)
                } else {
                    hir::Expr::fun_call(
                        func_ref_env_ref(),
                        vec![arg_ref_env(), hir::Expr::number(idx as i64)],
                    )
                }
            }
            hir::Expr::FuncRef(_) => e,
            hir::Expr::OpCall(op, lhs, rhs) => {
                let l = self.compile_value_expr(*lhs, false)?;
                let r = self.compile_value_expr(*rhs, false)?;
                hir::Expr::op_call(op, l, r)
            }
            hir::Expr::FunCall(fexpr, arg_exprs) => {
                let new_fexpr = self.compile_value_expr(*fexpr, false)?;
                let new_args = arg_exprs
                    .into_iter()
                    .map(|x| self.compile_value_expr(x, false))
                    .collect::<Result<Vec<_>>>()?;
                let fun_ty = new_fexpr.1.as_fun_ty();
                // No need to create a new chapter if on_return is true.
                // In that case the args are modified later (see hir::Expr::Return)
                if fun_ty.asyncness.is_async() && !on_return {
                    self.compile_async_call(new_fexpr, new_args)?
                } else {
                    hir::Expr::fun_call(new_fexpr, new_args)
                }
            }
            hir::Expr::Assign(name, rhs) => {
                hir::Expr::assign(name, self.compile_value_expr(*rhs, false)?)
            }
            hir::Expr::If(cond_expr, then_exprs, else_exprs) => {
                return self.compile_if(&e.1, *cond_expr, then_exprs, else_exprs);
            }
            hir::Expr::Yield(expr) => {
                let new_expr = self.compile_value_expr(*expr, false)?;
                hir::Expr::yield_(new_expr)
            }
            hir::Expr::While(_cond_expr, _body_exprs) => todo!(),
            hir::Expr::Alloc(_) => e,
            hir::Expr::Return(expr) => {
                let new_expr = self.compile_value_expr(*expr, true)?;
                hir::Expr::return_(new_expr)
            }
            hir::Expr::CondReturn(cond, fexpr_t, args_t, fexpr_f, args_f) => {
                let new_cond = self.compile_value_expr(*cond, false)?;
                let new_fexpr_t = self.compile_value_expr(*fexpr_t, false)?;
                let new_fexpr_f = self.compile_value_expr(*fexpr_f, false)?;
                let new_args_t = args_t
                    .into_iter()
                    .map(|x| self.compile_value_expr(x, false))
                    .collect::<Result<Vec<_>>>()?;
                let new_args_f = args_f
                    .into_iter()
                    .map(|x| self.compile_value_expr(x, false))
                    .collect::<Result<Vec<_>>>()?;
                hir::Expr::cond_return(new_cond, new_fexpr_t, new_args_t, new_fexpr_f, new_args_f)
            }
            hir::Expr::Branch(fname, expr) => {
                let if_result = self.compile_value_expr(*expr, false)?;
                hir::Expr::branch(fname, if_result)
            }
            hir::Expr::EnvRef(idx) => hir::Expr::fun_call(
                func_ref_env_ref(),
                vec![arg_ref_env(), hir::Expr::number(idx as i64)],
            ),
            _ => panic!("[BUG] unexpected for async_splitter: {:?}", e.0),
        };
        Ok(Some(new_e))
    }

    /// On calling an async function, create a new chapter and
    /// append the async call to the current chapter
    fn compile_async_call(
        &mut self,
        fexpr: hir::TypedExpr,
        mut new_args: Vec<hir::TypedExpr>,
    ) -> Result<hir::TypedExpr> {
        let hir::Ty::Fun(fun_ty) = &fexpr.1 else {
            return Err(anyhow!("[BUG] not a function: {:?}", fexpr.0));
        };
        // Append `$env` and `$cont` (i.e. the next chapter)
        new_args.insert(0, arg_ref_env());
        let next_chapter = {
            let next_chapter_name = chapter_func_name(&self.orig_func.name, self.chapters.len());
            let next_chapter_ty = hir::FunTy {
                asyncness: hir::Asyncness::Lowered,
                param_tys: vec![hir::Ty::ChiikaEnv, *fun_ty.ret_ty.clone()],
                ret_ty: Box::new(hir::Ty::RustFuture),
            };
            hir::Expr::func_ref(next_chapter_name, next_chapter_ty)
        };
        new_args.push(next_chapter);

        // Change chapter here
        let async_result_ty = *fun_ty.ret_ty.clone();
        let last_chapter = self.chapters.last_mut();
        let terminator = hir::Expr::async_call(fexpr, new_args);
        last_chapter.stmts.push(terminator);
        last_chapter.async_result_ty = Some(async_result_ty.clone());
        self.chapters.add(Chapter::new(chapter_func_name(
            &self.orig_func.name,
            self.chapters.len(),
        )));

        Ok(arg_ref_async_result(async_result_ty))
    }

    fn compile_if(
        &mut self,
        if_ty: &hir::Ty,
        cond_expr: hir::TypedExpr,
        then_exprs: Vec<hir::TypedExpr>,
        else_exprs: Vec<hir::TypedExpr>,
    ) -> Result<Option<hir::TypedExpr>> {
        let func_name = self.chapters.current_name().to_string();

        let new_cond_expr = self.compile_value_expr(cond_expr, false)?;
        let mut then_chap = Chapter::new_suffixed(&func_name, "t");
        let mut else_chap = Chapter::new_suffixed(&func_name, "f");
        // Statements after `if` goes to an "endif" chapter
        let endif_params = vec![hir::Param {
            name: "$ifResult".to_string(),
            ty: if_ty.clone(),
        }];
        let endif_chap = Chapter::new_suffixed(&func_name, "e"); // e for endif

        self.compile_if_clause(&mut then_chap, then_exprs, &endif_chap.name)?;
        self.compile_if_clause(&mut else_chap, else_exprs, &endif_chap.name)?;

        let (fexpr_t, args_t) = self.goto_call(&then_chap.name);
        let (fexpr_f, args_f) = self.goto_call(&else_chap.name);
        let terminator = hir::Expr::cond_return(new_cond_expr, fexpr_t, args_t, fexpr_f, args_f);
        self.chapters.add_stmt(terminator);
        self.chapters.add(then_chap);
        self.chapters.add(else_chap);
        if *if_ty == hir::Ty::Void {
            // Both branches end with return
            Ok(None)
        } else {
            self.chapters.add(endif_chap);
            // FIXME: This magic number is decided by async_splitter.rs
            Ok(Some(hir::Expr::arg_ref(1, if_ty.clone())))
        }
    }

    fn compile_if_clause(
        &mut self,
        clause_chap: &mut Chapter,
        mut exprs: Vec<hir::TypedExpr>,
        endif_chap_name: &str,
    ) -> Result<()> {
        let e = exprs.pop().unwrap();
        let opt_vexpr = match e {
            (hir::Expr::Return(_), _) => {
                exprs.push(e);
                None
            }
            (hir::Expr::Yield(vexpr), _) => Some(vexpr),
            _ => {
                return Err(anyhow!(
                    "[BUG] last statement of a clause must be a yield or a return"
                ))
            }
        };
        for expr in exprs {
            if let Some(new_expr) = self.compile_expr(expr, false)? {
                clause_chap.add_stmt(new_expr);
            }
        }
        if let Some(vexpr) = opt_vexpr {
            let new_vexpr = self.compile_value_expr(*vexpr, false)?;
            let goto_endif = hir::Expr::branch(endif_chap_name, new_vexpr);
            clause_chap.add_stmt(goto_endif);
        }
        Ok(())
    }

    /// Generate a call to the chapter function
    fn goto_call(&self, chap_name: &str) -> (hir::TypedExpr, Vec<hir::TypedExpr>) {
        // TODO: Support local variables
        //let mut args = self
        //    .allocs
        //    .iter()
        //    .map(|(name, ty)| hir::Expr::lvar_ref(name.clone(), ty.clone()))
        //    .collect::<Vec<_>>();
        let chap_fun_ty = {
            //let mut param_tys = self
            //    .allocs
            //    .iter()
            //    .map(|(_, ty)| ty.clone())
            //    .collect::<Vec<_>>();
            hir::FunTy {
                asyncness: self.orig_func.asyncness.clone(),
                param_tys: vec![],
                ret_ty: Box::new(self.orig_func.ret_ty.clone()),
            }
        };
        (hir::Expr::func_ref(chap_name, chap_fun_ty), vec![])
    }
}

// Convert `Fun((X)->Y)` to `Fun((ChiikaEnv, X, Fun((Y,ChiikaEnv)->RustFuture))->RustFuture)`
fn async_fun_ty(orig_fun_ty: &hir::FunTy) -> hir::FunTy {
    let mut param_tys = orig_fun_ty.param_tys.clone();
    param_tys.insert(0, hir::Ty::ChiikaEnv);
    param_tys.push(hir::Ty::Fun(hir::FunTy {
        asyncness: hir::Asyncness::Lowered,
        param_tys: vec![hir::Ty::ChiikaEnv, *orig_fun_ty.ret_ty.clone()],
        ret_ty: Box::new(hir::Ty::RustFuture),
    }));
    hir::FunTy {
        asyncness: hir::Asyncness::Async,
        param_tys,
        ret_ty: Box::new(hir::Ty::RustFuture),
    }
}

fn prepend_env_to_fn_ty(fun_ty: &hir::FunTy) -> hir::FunTy {
    let mut param_tys = fun_ty.param_tys.clone();
    param_tys.insert(0, hir::Ty::ChiikaEnv);
    hir::FunTy {
        asyncness: hir::Asyncness::Lowered,
        param_tys,
        ret_ty: Box::new(hir::Ty::RustFuture),
    }
}

/// Append params for async (`$env` and `$cont`)
fn append_async_params(
    params: &[hir::Param],
    result_ty: hir::Ty,
    generated: bool,
) -> Vec<hir::Param> {
    let mut new_params = params.to_vec();
    if generated {
        new_params.insert(0, hir::Param::new(hir::Ty::ChiikaEnv, "$env"));
    } else {
        new_params.insert(0, hir::Param::new(hir::Ty::ChiikaEnv, "$env"));
        let fun_ty = hir::FunTy {
            asyncness: hir::Asyncness::Lowered,
            param_tys: vec![hir::Ty::ChiikaEnv, result_ty],
            ret_ty: Box::new(hir::Ty::RustFuture),
        };
        new_params.push(hir::Param::new(hir::Ty::Fun(fun_ty), "$cont"));
    }

    new_params
}

/// Prepend successive calls of `chiika_env_push` to `stmts`
fn prepend_async_intro(
    orig_func: &hir::Function,
    mut stmts: Vec<hir::TypedExpr>,
) -> Vec<hir::TypedExpr> {
    let mut push_items = vec![arg_ref_cont(
        orig_func.params.len(),
        orig_func.ret_ty.clone(),
    )];
    for i in (0..orig_func.params.len()).rev() {
        push_items.push(hir::Expr::arg_ref(i + 1, orig_func.params[i].ty.clone()));
        // +1 for $env
    }
    let mut push_calls = push_items
        .into_iter()
        .map(|arg| call_chiika_env_push(arg, orig_func))
        .collect::<Vec<_>>();
    push_calls.append(&mut stmts);
    push_calls
}

fn modify_cond_call(
    mut fexpr: hir::TypedExpr,
    mut args: Vec<hir::TypedExpr>,
    orig_func: &hir::Function,
) -> hir::TypedExpr {
    let get_env = arg_ref_env();
    if fexpr.1.is_async_fun() {
        fexpr.1 = prepend_env_to_fn_ty(&fexpr.1.into()).into();
        args.insert(0, get_env);
        hir::Expr::fun_call(fexpr, args)
    } else {
        let sync_call = hir::Expr::fun_call(fexpr, args);
        hir::Expr::fun_call(
            arg_ref_cont(orig_func.params.len(), orig_func.ret_ty.clone()),
            vec![get_env, sync_call],
        )
    }
}

/// Create name of generated function like `foo_1`
fn chapter_func_name(orig_name: &str, chapter_idx: usize) -> String {
    format!("{}_{}", orig_name, chapter_idx)
}

/// Get the `$env` that is 0-th param of async func
fn arg_ref_env() -> hir::TypedExpr {
    hir::Expr::arg_ref(0, hir::Ty::ChiikaEnv)
}

/// Get the `$cont` param of async func
/// The continuation takes an argument.
fn arg_ref_cont(arity: usize, arg_ty: hir::Ty) -> hir::TypedExpr {
    let cont_ty = hir::FunTy {
        asyncness: hir::Asyncness::Lowered,
        param_tys: vec![hir::Ty::ChiikaEnv, arg_ty],
        ret_ty: Box::new(hir::Ty::RustFuture),
    };
    hir::Expr::arg_ref(arity + 1, hir::Ty::Fun(cont_ty))
}

/// Get the `$async_result` which is 1-th param of chapter func
fn arg_ref_async_result(ty: hir::Ty) -> hir::TypedExpr {
    hir::Expr::arg_ref(1, ty)
}

fn call_chiika_env_pop(
    n_pop: usize,
    popped_value_ty: hir::Ty,
    current_func_arity: usize,
) -> hir::TypedExpr {
    let env_pop = {
        let fun_ty = hir::FunTy {
            asyncness: hir::Asyncness::Lowered,
            param_tys: vec![hir::Ty::ChiikaEnv, hir::Ty::Int],
            ret_ty: Box::new(hir::Ty::Any),
        };
        hir::Expr::func_ref("chiika_env_pop", fun_ty)
    };
    let cast_type = match &popped_value_ty {
        hir::Ty::Int => hir::CastType::AnyToInt,
        hir::Ty::Fun(fun_ty) => hir::CastType::AnyToFun(fun_ty.clone()),
        _ => panic!("[BUG] cannot cast: {:?}", popped_value_ty),
    };
    hir::Expr::cast(
        cast_type,
        hir::Expr::fun_call(
            env_pop,
            vec![arg_ref_env(), hir::Expr::number(n_pop as i64)],
        ),
    )
}

fn call_chiika_env_push(val: hir::TypedExpr, orig_func: &hir::Function) -> hir::TypedExpr {
    let cast_val = {
        let cast_type = match val.1 {
            hir::Ty::Null => hir::CastType::NullToAny,
            hir::Ty::Int => hir::CastType::IntToAny,
            hir::Ty::Fun(_) => hir::CastType::FunToAny,
            _ => panic!("[BUG] don't know how to cast {:?} to Any", val),
        };
        hir::Expr::cast(cast_type, val)
    };
    let fun_ty = hir::FunTy {
        asyncness: hir::Asyncness::Lowered,
        param_tys: vec![hir::Ty::ChiikaEnv, hir::Ty::Any],
        ret_ty: Box::new(hir::Ty::Null),
    };
    hir::Expr::fun_call(
        hir::Expr::func_ref("chiika_env_push", fun_ty),
        vec![arg_ref_env(), cast_val],
    )
}

fn func_ref_env_ref() -> hir::TypedExpr {
    let fun_ty = hir::FunTy {
        asyncness: hir::Asyncness::Lowered,
        param_tys: vec![hir::Ty::ChiikaEnv, hir::Ty::Int],
        // Milika lvars are all int
        ret_ty: Box::new(hir::Ty::Int),
    };
    hir::Expr::func_ref("chiika_env_ref", fun_ty)
}

#[derive(Debug, Default)]
struct Chapters {
    chaps: Vec<Chapter>,
}

impl Chapters {
    fn new() -> Chapters {
        Chapters { chaps: vec![] }
    }

    fn clear(&mut self) {
        self.chaps.clear();
    }

    fn extract(&mut self) -> VecDeque<Chapter> {
        self.chaps.drain(..).collect()
    }

    fn len(&self) -> usize {
        self.chaps.len()
    }

    fn last_mut(&mut self) -> &mut Chapter {
        self.chaps.last_mut().unwrap()
    }

    /// Returns the name of the last chapter
    fn current_name(&self) -> &str {
        &self.chaps.last().unwrap().name
    }

    fn add(&mut self, chap: Chapter) {
        self.chaps.push(chap);
    }

    fn add_stmt(&mut self, stmt: hir::TypedExpr) {
        self.chaps.last_mut().unwrap().add_stmt(stmt);
    }
}

#[derive(Debug)]
struct Chapter {
    stmts: Vec<hir::TypedExpr>,
    // The resulting type of the async function called with the last stmt
    async_result_ty: Option<hir::Ty>,
    name: String,
}

impl Chapter {
    fn new(name: String) -> Chapter {
        Chapter {
            stmts: vec![],
            async_result_ty: None,
            name,
        }
    }

    fn new_suffixed(base_name: &str, suffix: &str) -> Chapter {
        Chapter::new(format!("{}'{}", base_name, suffix))
    }

    fn add_stmt(&mut self, stmt: hir::TypedExpr) {
        self.stmts.push(stmt);
    }
}
