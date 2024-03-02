use crate::hir;
use anyhow::{anyhow, bail, Result};
use std::collections::VecDeque;

#[derive(Debug)]
struct AsyncSplitter {
    chapters: VecDeque<Chapter>,
}

#[derive(Debug)]
struct Chapter {
    stmts: Vec<hir::TypedExpr>,
    // The resulting type of the async function called with the lhir stmt
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
pub fn run(hir: hir::Program) -> Result<hir::Program> {
    let mut c = AsyncSplitter {
        chapters: Default::default(),
    };
    let mut funcs = vec![];
    for f in hir.funcs {
        let mut split_funcs = c.compile_func(f)?;
        funcs.append(&mut split_funcs);
    }
    Ok(hir::Program { funcs, ..hir })
}

impl AsyncSplitter {
    fn compile_func(&mut self, mut f: hir::Function) -> Result<Vec<hir::Function>> {
        self.chapters.clear();
        self.chapters.push_back(Chapter::new());
        for expr in f.body_stmts.drain(..).collect::<Vec<_>>() {
            let new_expr = self.compile_expr(&f, expr)?;
            self.chapters.back_mut().unwrap().stmts.push(new_expr);
        }

        if self.chapters.len() == 1 {
            // Has no async call; no modification needed
            Ok(vec![hir::Function {
                name: f.name,
                params: f.params.into_iter().map(|x| x.into()).collect(),
                ret_ty: f.ret_ty.into(),
                body_stmts: self.chapters.pop_front().unwrap().stmts,
            }])
        } else {
            let chaps = self.chapters.drain(..).collect();
            self.generate_split_funcs(f, chaps)
        }
    }

    fn generate_split_funcs(
        &mut self,
        orig_func: hir::Function,
        mut chapters: VecDeque<Chapter>,
    ) -> Result<Vec<hir::Function>> {
        let n_chapters = chapters.len();
        let mut i = 0;
        let mut lhir_chap_result_ty = None;
        let mut split_funcs = vec![];
        while let Some(chap) = chapters.pop_front() {
            let new_func = if i == 0 {
                hir::Function {
                    name: orig_func.name.clone(),
                    params: prepend_async_params(
                        &orig_func
                            .params
                            .iter()
                            .map(|x| x.clone().into())
                            .collect::<Vec<_>>(),
                        orig_func.ret_ty.clone().into(),
                    ),
                    ret_ty: hir::Ty::RustFuture,
                    body_stmts: prepend_async_intro(&orig_func, chap.stmts),
                }
            } else {
                hir::Function {
                    name: chapter_func_name(&orig_func.name, i),
                    params: vec![
                        hir::Param::new(hir::Ty::ChiikaEnv, "$env"),
                        hir::Param::new(lhir_chap_result_ty.unwrap(), "$async_result"),
                    ],
                    ret_ty: hir::Ty::RustFuture,
                    body_stmts: if i == n_chapters - 1 {
                        append_async_outro(&orig_func, chap.stmts, orig_func.ret_ty.clone().into())
                    } else {
                        chap.stmts
                    },
                }
            };
            i += 1;
            lhir_chap_result_ty = Some(chap.async_result_ty.unwrap());
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
    ) -> Result<hir::TypedExpr> {
        let new_e = match e.0 {
            hir::Expr::Number(n) => e,
            hir::Expr::LVarRef(ref name) => {
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
                    // The variable is just there in the first chapter
                    e
                } else {
                    hir::Expr::fun_call(
                        func_ref_env_ref(),
                        vec![arg_ref_env(), hir::Expr::number(idx as i64)],
                    )
                }
            }
            hir::Expr::FuncRef(_) => e,
            hir::Expr::OpCall(op, lhs, rhs) => {
                let l = self.compile_expr(orig_func, lhs)?;
                let r = self.compile_expr(orig_func, rhs)?;
                hir::Expr::OpCall(op, Box::new(l), Box::new(r))
            }
            hir::Expr::FunCall(fexpr, arg_exprs) => {
                let mut new_args = arg_exprs
                    .into_iter()
                    .map(|(x, _)| self.compile_expr(orig_func, x))
                    .collect::<Result<Vec<_>>>()?;
                let hir::Ty::Fun(fun_ty) = e.1 else {
                    return Err(anyhow!("[BUG] not a function: {:?}", e.1));
                };
                if fun_ty.is_async {
                    new_args.insert(0, arg_ref_env());
                    new_args.insert(
                        1,
                        hir::Expr::func_ref(chapter_func_name(
                            &orig_func.name,
                            self.chapters.len(),
                        )),
                    );
                    let cps_call = hir::Expr::FunCall(Box::new(fexpr), new_args);

                    // Change chapter here
                    let async_result_ty = (*fun_ty.ret_ty).clone().into();
                    let lhir_chapter = self.chapters.back_mut().unwrap();
                    lhir_chapter.stmts.push(cps_call);
                    lhir_chapter.async_result_ty = Some(async_result_ty.clone());
                    self.chapters.push_back(Chapter::new());

                    arg_ref_async_result(async_result_ty)
                } else {
                    hir::Expr::FunCall(Box::new(fexpr), new_args)
                }
            }
            hir::Expr::Assign(name, rhs) => {
                hir::Expr::Assign(name, Box::new(self.compile_expr(orig_func, rhs.0)?))
            }
            //hir::Expr::While(cond_expr, body_exprs) => todo!(),
            //hir::Expr::If(cond_expr, then_exprs, else_exprs) => todo!(),
            hir::Expr::Alloc(name) => e,
            hir::Expr::Return(expr) => {
                hir::Expr::Return(Box::new(self.compile_expr(orig_func, expr.0)?))
            }
            _ => todo!("{:?}", e),
        };
        Ok(new_e)
    }
}

/// Prepend params for async
fn prepend_async_params(params: &[hir::Param], result_ty: hir::Ty) -> Vec<hir::Param> {
    let mut new_params = params.to_vec();
    new_params.insert(0, hir::Param::new(hir::Ty::ChiikaEnv, "$env"));

    let fun_ty = hir::FunTy {
        is_async: false, // chiika-1 does not have notion of asyncness
        param_tys: vec![hir::Ty::ChiikaEnv, result_ty],
        ret_ty: Box::new(hir::Ty::RustFuture),
    };
    new_params.insert(1, hir::Param::new(hir::Ty::Fun(fun_ty), "$cont"));

    new_params
}

/// Prepend successive calls of `chiika_env_push` to `stmts`
fn prepend_async_intro(
    orig_func: &hir::Function,
    mut stmts: Vec<hir::TypedExpr>,
) -> Vec<hir::TypedExpr> {
    let push_items = vec![hir::Expr::arg_ref("$cont")].into_iter().chain(
        orig_func
            .params
            .iter()
            .map(|param| hir::Expr::var_ref(&param.name)),
    );

    let mut push_calls = push_items
        .map(|arg| {
            let cast = hir::Expr::Cast(Box::new(arg), hir::Ty::Opaque);
            hir::Expr::FunCall(Box::new(func_ref_env_push()), vec![arg_ref_env(), cast])
        })
        .collect::<Vec<_>>();
    push_calls.append(&mut stmts);
    push_calls
}

/// Appends successive calls of `chiika_env_pop` and a call of original `$cont` to `stmts`
fn append_async_outro(
    orig_func: &hir::Function,
    mut stmts: Vec<hir::TypedExpr>,
    result_ty: hir::Ty,
) -> Vec<hir::TypedExpr> {
    let (hir::Expr::Return(ret_val), _) = stmts.pop().unwrap() else {
        bail!("TODO: async func must end with `return` now");
    };
    let cont = {
        let cont_ty = hir::FunTy {
            is_async: false,
            param_tys: vec![hir::Ty::ChiikaEnv, result_ty],
            ret_ty: Box::new(hir::Ty::RustFuture),
        };
        let env_pop = func_ref_env_pop(cont_ty.clone());
        let n_pop = orig_func.params.len() + 1; // +1 for $cont
        hir::Expr::fun_call(
            env_pop,
            vec![arg_ref_env(), hir::Expr::number(n_pop as i64)],
        )
    };
    stmts.push(hir::Expr::fun_call(cont, vec![arg_ref_env(), ret_val]));
    stmts
}

/// Create name of generated function like `foo_1`
fn chapter_func_name(orig_name: &str, chapter_idx: usize) -> String {
    format!("{}_{}", orig_name, chapter_idx)
}

/// Get the `$env` which is 0-th param of current func
fn arg_ref_env() -> hir::TypedExpr {
    hir::Expr::arg_ref(0, hir::Ty::ChiikaEnv)
}

/// Get the `$cont` which is 1-th param of current func
fn arg_ref_cont(result_ty: hir::Ty) -> hir::TypedExpr {
    let cont_ty = hir::FunTy {
        is_async: false,
        param_tys: vec![hir::Ty::ChiikaEnv, result_ty],
        ret_ty: Box::new(hir::Ty::RustFuture),
    };
    hir::Expr::arg_ref(1, hir::Ty::Fun(cont_ty))
}

fn arg_ref_async_result(ty: hir::Ty) -> hir::TypedExpr {
    hir::Expr::arg_ref("$async_result", ty)
}

fn func_ref_env_pop(cont_arg_ty: hir::Ty) -> hir::TypedExpr {
    let fun_ty = hir::FunTy {
        is_async: false,
        param_tys: vec![hir::Ty::ChiikaEnv, hir::Ty::Int],
        ret_ty: Box::new(cont_arg_ty),
    };
    hir::Expr::func_ref("chiika_env_pop", fun_ty)
}

fn func_ref_env_push() -> hir::TypedExpr {
    let fun_ty = hir::FunTy {
        is_async: false,
        param_tys: vec![hir::Ty::ChiikaEnv, hir::Ty::Opaque],
        ret_ty: Box::new(hir::Ty::Int),
    };
    hir::Expr::func_ref("chiika_env_push", fun_ty)
}

fn func_ref_env_ref() -> hir::TypedExpr {
    let fun_ty = hir::FunTy {
        is_async: false,
        param_tys: vec![hir::Ty::ChiikaEnv, hir::Ty::Int],
        // Milika lvars are all int
        ret_ty: Box::new(hir::Ty::Int),
    };
    hir::Expr::func_ref("chiika_env_ref", fun_ty)
}
