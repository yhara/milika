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

// `$env` and `$cont` are prepended to the original params
const N_ASYNC_PARAMS: usize = 2;

#[derive(Debug)]
struct AsyncSplitter {
    // True if the original Milika function (i.e. the function at the beginning of the group) is async
    origin_is_async: bool,
    // True if compiling the last function in the group
    last_in_group: bool,
    chapters: VecDeque<Chapter>,
}

#[derive(Debug)]
struct Chapter {
    stmts: Vec<hir::TypedExpr>,
    // The resulting type of the async function called with the last stmt
    async_result_ty: Option<hir::Ty>,
}

impl Chapter {
    fn new() -> Chapter {
        Chapter {
            stmts: vec![],
            async_result_ty: None,
        }
    }
}

/// Splits asynchronous Milika func into multiple funcs.
/// Also, signatures of async externs are modified to take `$env` and `$cont` as the first two params.
pub fn run(shir: hir::split::Program) -> Result<hir::split::Program> {
    let externs = shir
        .externs
        .into_iter()
        .map(|e| {
            if e.is_async {
                hir::Extern {
                    params: prepend_async_params(&e.params, e.ret_ty.clone(), false),
                    ret_ty: hir::Ty::RustFuture,
                    ..e
                }
            } else {
                e
            }
        })
        .collect();

    let mut c = AsyncSplitter {
        origin_is_async: false,
        last_in_group: false,
        chapters: Default::default(),
    };
    let mut funcs = vec![];
    for group in shir.funcs {
        let group_size = group.len();
        let origin_is_async = group.first().unwrap().asyncness.is_async();
        for (i, f) in group.into_iter().enumerate() {
            let last_in_group = i == group_size - 1;
            let split_funcs = c.compile_func(f, last_in_group, origin_is_async)?;
            funcs.push(split_funcs);
        }
    }
    Ok(hir::split::Program::new(externs, funcs))
}

impl AsyncSplitter {
    fn compile_func(
        &mut self,
        mut f: hir::Function,
        last_in_group: bool,
        origin_is_async: bool,
    ) -> Result<Vec<hir::Function>> {
        self.origin_is_async = origin_is_async;
        self.last_in_group = last_in_group;
        self.chapters.clear();
        self.chapters.push_back(Chapter::new());
        for expr in f.body_stmts.drain(..).collect::<Vec<_>>() {
            let new_expr = self.compile_expr(&f, expr, false)?;
            self.chapters.back_mut().unwrap().stmts.push(new_expr);
        }

        if f.asyncness.is_async() {
            let chaps = self.chapters.drain(..).collect();
            self._generate_split_funcs(f, chaps)
        } else {
            // Has no async call; no modification needed
            Ok(vec![hir::Function {
                generated: f.generated,
                asyncness: hir::Asyncness::Lowered,
                name: f.name,
                params: f.params.into_iter().map(|x| x.into()).collect(),
                ret_ty: f.ret_ty.into(),
                body_stmts: self.chapters.pop_front().unwrap().stmts,
            }])
        }
    }

    /// Serialize `chapters` to functions
    fn _generate_split_funcs(
        &mut self,
        orig_func: hir::Function,
        mut chapters: VecDeque<Chapter>,
    ) -> Result<Vec<hir::Function>> {
        let mut i = 0;
        let mut last_chap_result_ty = None;
        let mut split_funcs = vec![];
        while let Some(chap) = chapters.pop_front() {
            let new_func = if i == 0 {
                let body_stmts = if orig_func.asyncness.is_async() && !orig_func.generated {
                    prepend_async_intro(&orig_func, chap.stmts)
                } else {
                    chap.stmts
                };
                // Entry point function has the same name as the original.
                hir::Function {
                    generated: orig_func.generated,
                    asyncness: orig_func.asyncness.clone(),
                    name: orig_func.name.clone(),
                    // It takes `$env` and `$cont` before the original params
                    params: prepend_async_params(
                        &orig_func
                            .params
                            .iter()
                            .map(|x| x.clone().into())
                            .collect::<Vec<_>>(),
                        orig_func.ret_ty.clone().into(),
                        orig_func.generated,
                    ),
                    ret_ty: hir::Ty::RustFuture,
                    body_stmts: modify_return(
                        self.origin_is_async,
                        &orig_func,
                        body_stmts,
                        self.last_in_group,
                    ),
                }
            } else {
                // The rest of the functions have a name like `foo_1`, `foo_2`, ...
                hir::Function {
                    generated: true,
                    asyncness: hir::Asyncness::Lowered,
                    name: chapter_func_name(&orig_func.name, i),
                    params: vec![
                        hir::Param::new(hir::Ty::ChiikaEnv, "$env"),
                        // The result of the previous async call
                        hir::Param::new(last_chap_result_ty.unwrap(), "$async_result"),
                    ],
                    ret_ty: hir::Ty::RustFuture,
                    body_stmts: modify_return(
                        self.origin_is_async,
                        &orig_func,
                        chap.stmts,
                        self.last_in_group,
                    ),
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

    /// Examine each expression for special care. Most important part is
    /// calling async function.
    fn compile_expr(
        &mut self,
        orig_func: &hir::Function,
        e: hir::TypedExpr,
        on_return: bool,
    ) -> Result<hir::TypedExpr> {
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
                    let new_idx = if orig_func.asyncness.is_async() {
                        idx + N_ASYNC_PARAMS
                    } else {
                        idx
                    };
                    hir::Expr::arg_ref(new_idx, e.1)
                } else {
                    hir::Expr::fun_call(
                        func_ref_env_ref(),
                        vec![arg_ref_env(), hir::Expr::number(idx as i64)],
                    )
                }
            }
            hir::Expr::FuncRef(_) => e,
            hir::Expr::OpCall(op, lhs, rhs) => {
                let l = self.compile_expr(orig_func, *lhs, false)?;
                let r = self.compile_expr(orig_func, *rhs, false)?;
                hir::Expr::op_call(op, l, r)
            }
            hir::Expr::FunCall(fexpr, arg_exprs) => {
                let new_fexpr = self.compile_expr(orig_func, *fexpr, false)?;
                let new_args = arg_exprs
                    .into_iter()
                    .map(|x| self.compile_expr(orig_func, x, false))
                    .collect::<Result<Vec<_>>>()?;
                let fun_ty = new_fexpr.1.as_fun_ty();
                // No need to create a new chapter if on_return is true.
                // In that case the args are modified later (see hir::Expr::Return)
                if fun_ty.asyncness.is_async() && !on_return {
                    self.compile_async_call(orig_func, new_fexpr, new_args)?
                } else {
                    hir::Expr::fun_call(new_fexpr, new_args)
                }
            }
            hir::Expr::Assign(name, rhs) => {
                hir::Expr::assign(name, self.compile_expr(orig_func, *rhs, false)?)
            }
            hir::Expr::If(cond_expr, then_exprs, else_exprs) => {
                let new_cond = self.compile_expr(orig_func, *cond_expr, false)?;
                let new_then = then_exprs
                    .into_iter()
                    .map(|e| self.compile_expr(orig_func, e, false))
                    .collect::<Result<Vec<_>>>()?;
                let new_else = else_exprs
                    .into_iter()
                    .map(|e| self.compile_expr(orig_func, e, false))
                    .collect::<Result<Vec<_>>>()?;
                hir::Expr::if_(new_cond, new_then, new_else)
            }
            hir::Expr::Yield(expr) => {
                let new_expr = self.compile_expr(orig_func, *expr, false)?;
                hir::Expr::yield_(new_expr)
            }
            hir::Expr::While(_cond_expr, _body_exprs) => todo!(),
            hir::Expr::Alloc(_) => e,
            hir::Expr::Return(expr) => {
                let new_expr = self.compile_expr(orig_func, *expr, true)?;
                hir::Expr::return_(new_expr)
            }
            hir::Expr::CondReturn(cond, fexpr_t, args_t, fexpr_f, args_f) => {
                let new_cond = self.compile_expr(orig_func, *cond, false)?;
                let new_fexpr_t = self.compile_expr(orig_func, *fexpr_t, false)?;
                let new_fexpr_f = self.compile_expr(orig_func, *fexpr_f, false)?;
                let new_args_t = args_t
                    .into_iter()
                    .map(|x| self.compile_expr(orig_func, x, false))
                    .collect::<Result<Vec<_>>>()?;
                let new_args_f = args_f
                    .into_iter()
                    .map(|x| self.compile_expr(orig_func, x, false))
                    .collect::<Result<Vec<_>>>()?;
                hir::Expr::cond_return(new_cond, new_fexpr_t, new_args_t, new_fexpr_f, new_args_f)
            }
            hir::Expr::Branch(fname, expr) => {
                let if_result = self.compile_expr(orig_func, *expr, false)?;
                hir::Expr::branch(fname, if_result)
            }
            hir::Expr::GetIfResult => hir::Expr::arg_ref(1, e.1),
            _ => panic!("[BUG] unexpected for async_splitter: {:?}", e.0),
        };
        Ok(new_e)
    }

    /// On calling an async function, create a new chapter and
    /// append the async call to the current chapter
    fn compile_async_call(
        &mut self,
        orig_func: &hir::Function,
        fexpr: hir::TypedExpr,
        mut new_args: Vec<hir::TypedExpr>,
    ) -> Result<hir::TypedExpr> {
        let hir::Ty::Fun(fun_ty) = &fexpr.1 else {
            return Err(anyhow!("[BUG] not a function: {:?}", fexpr.0));
        };
        // Prepend `$env` and `$cont` (i.e. the next chapter)
        new_args.insert(0, arg_ref_env());
        let next_chapter = {
            let next_chapter_name = chapter_func_name(&orig_func.name, self.chapters.len());
            let next_chapter_ty = hir::FunTy {
                asyncness: hir::Asyncness::Lowered,
                param_tys: vec![hir::Ty::ChiikaEnv, *fun_ty.ret_ty.clone()],
                ret_ty: Box::new(hir::Ty::RustFuture),
            };
            hir::Expr::func_ref(next_chapter_name, next_chapter_ty)
        };
        new_args.insert(1, next_chapter);

        // Change chapter here
        let async_result_ty = *fun_ty.ret_ty.clone();
        let last_chapter = self.chapters.back_mut().unwrap();
        let terminator = hir::Expr::async_call(fexpr, new_args);
        //            hir::Expr::return_(hir::Expr::fun_call(
        //            (fexpr.0, async_fun_ty(fun_ty).into()),
        //            new_args,
        //        ));
        last_chapter.stmts.push(terminator);
        last_chapter.async_result_ty = Some(async_result_ty.clone());
        self.chapters.push_back(Chapter::new());

        Ok(arg_ref_async_result(async_result_ty))
    }
}

// Convert `Fun((X)->Y)` to `Fun((ChiikaEnv, Fun((ChiikaEnv,Y)->RustFuture), X)->RustFuture)`
fn async_fun_ty(orig_fun_ty: &hir::FunTy) -> hir::FunTy {
    let mut param_tys = orig_fun_ty.param_tys.clone();
    param_tys.insert(0, hir::Ty::ChiikaEnv);
    param_tys.insert(
        1,
        hir::Ty::Fun(hir::FunTy {
            asyncness: hir::Asyncness::Lowered,
            param_tys: vec![hir::Ty::ChiikaEnv, *orig_fun_ty.ret_ty.clone()],
            ret_ty: Box::new(hir::Ty::RustFuture),
        }),
    );
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

/// Prepend params for async (`$env` and `$cont`)
fn prepend_async_params(
    params: &[hir::Param],
    result_ty: hir::Ty,
    generated: bool,
) -> Vec<hir::Param> {
    let mut new_params = params.to_vec();
    new_params.insert(0, hir::Param::new(hir::Ty::ChiikaEnv, "$env"));

    if !generated {
        let fun_ty = hir::FunTy {
            asyncness: hir::Asyncness::Lowered,
            param_tys: vec![hir::Ty::ChiikaEnv, result_ty],
            ret_ty: Box::new(hir::Ty::RustFuture),
        };
        new_params.insert(1, hir::Param::new(hir::Ty::Fun(fun_ty), "$cont"));
    }

    new_params
}

/// Prepend successive calls of `chiika_env_push` to `stmts`
fn prepend_async_intro(
    orig_func: &hir::Function,
    mut stmts: Vec<hir::TypedExpr>,
) -> Vec<hir::TypedExpr> {
    let push_items = vec![arg_ref_cont(orig_func.ret_ty.clone())]
        .into_iter()
        .chain(
            orig_func
                .params
                .iter()
                .enumerate()
                .map(|(i, param)| hir::Expr::arg_ref(N_ASYNC_PARAMS + i, param.ty.clone())),
        );

    let mut push_calls = push_items
        .map(|arg| call_chiika_env_push(arg))
        .collect::<Vec<_>>();
    push_calls.append(&mut stmts);
    push_calls
}

/// Insert call of `chiika_env_pop` before the `return`.
fn modify_return(
    origin_is_async: bool,
    orig_func: &hir::Function,
    mut body_stmts: Vec<hir::TypedExpr>,
    last_in_group: bool,
) -> Vec<hir::TypedExpr> {
    if !origin_is_async {
        // No mod needed for a sync function
        return body_stmts;
    }
    let new_ret_value = match body_stmts.pop() {
        None => panic!("function body is empty"),
        // Jump to if-branch-functions
        Some((hir::Expr::CondReturn(cond_expr, fexpr_t, args_t, fexpr_f, args_f), _)) => {
            let call_t = modify_cond_call(*fexpr_t, args_t, orig_func.ret_ty.clone());
            let call_f = modify_cond_call(*fexpr_f, args_f, orig_func.ret_ty.clone());
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
        Some((hir::Expr::Return(value_expr), _)) => {
            debug_assert!(last_in_group);
            let env_pop = {
                // FIXME: This is not always correct. Should be the number of params of the group first function
                let n_pop = orig_func.params.len() + 1; // +1 for $cont
                let cont_ty = hir::Ty::Fun(hir::FunTy {
                    asyncness: hir::Asyncness::Lowered,
                    param_tys: vec![hir::Ty::ChiikaEnv, orig_func.ret_ty.clone()],
                    ret_ty: Box::new(hir::Ty::RustFuture),
                });
                call_chiika_env_pop(n_pop, cont_ty)
            };
            if value_expr.0.is_async_fun_call() {
                // Convert `callee(args...)`
                // to `callee(env, env_pop(), args...)`
                let hir::Expr::FunCall(fexpr, mut args) = value_expr.0 else {
                    unreachable!();
                };
                args.insert(0, arg_ref_env());
                args.insert(1, env_pop);
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

fn modify_cond_call(
    mut fexpr: hir::TypedExpr,
    mut args: Vec<hir::TypedExpr>,
    orig_ret_ty: hir::Ty,
) -> hir::TypedExpr {
    if fexpr.1.is_async_fun() {
        fexpr.1 = prepend_env_to_fn_ty(&fexpr.1.into()).into();
        args.insert(0, arg_ref_env());
        hir::Expr::fun_call(fexpr, args)
    } else {
        let sync_call = hir::Expr::fun_call(fexpr, args);
        hir::Expr::fun_call(arg_ref_cont(orig_ret_ty), vec![arg_ref_env(), sync_call])
    }
}

/// Create name of generated function like `foo_1`
fn chapter_func_name(orig_name: &str, chapter_idx: usize) -> String {
    format!("{}_{}", orig_name, chapter_idx)
}

/// Get the `$env` which is 0-th param of async func
fn arg_ref_env() -> hir::TypedExpr {
    hir::Expr::arg_ref(0, hir::Ty::ChiikaEnv)
}

/// Get the `$cont` which is 1-th param of async func
/// The continuation takes an argument.
fn arg_ref_cont(arg_ty: hir::Ty) -> hir::TypedExpr {
    let cont_ty = hir::FunTy {
        asyncness: hir::Asyncness::Lowered,
        param_tys: vec![hir::Ty::ChiikaEnv, arg_ty],
        ret_ty: Box::new(hir::Ty::RustFuture),
    };
    hir::Expr::arg_ref(1, hir::Ty::Fun(cont_ty))
}

/// Get the `$async_result` which is 1-th param of chapter func
fn arg_ref_async_result(ty: hir::Ty) -> hir::TypedExpr {
    hir::Expr::arg_ref(1, ty)
}

fn call_chiika_env_pop(n_pop: usize, popped_value_ty: hir::Ty) -> hir::TypedExpr {
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

fn call_chiika_env_push(val: hir::TypedExpr) -> hir::TypedExpr {
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
