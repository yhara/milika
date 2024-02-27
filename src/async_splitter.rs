use crate::ast;
use crate::asyncness_check;
use crate::hir;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
struct Ast2Hir {
    sigs: HashMap<String, ast::FunTy>,
    chapters: VecDeque<Chapter>,
}

#[derive(Debug)]
struct Chapter {
    stmts: Vec<hir::Expr>,
    // The resulting type of the async function called with the last stmt
    async_result_ty: hir::Ty,
}

impl Chapter {
    fn new() -> Chapter {
        Chapter {
            stmts: vec![],
            async_result_ty: hir::Ty::Raw("[BUG] async_result_ty not set".to_string()),
        }
    }
}

/// Converts ast into hir.
/// Also, splits asynchronous Milika func into multiple funcs.
pub fn run(ast: ast::Program) -> Result<hir::Program> {
    let mut c = Ast2Hir {
        sigs: asyncness_check::gather_sigs(&ast.0)?,
        chapters: Default::default(),
    };
    let mut externs = vec![];
    let mut funcs = vec![];
    for decl in ast.0 {
        match decl {
            ast::Declaration::Extern((e, _)) => externs.push(e.into()),
            ast::Declaration::Function((f, _)) => {
                let mut split_funcs = c.compile_func(f)?;
                funcs.append(&mut split_funcs);
            }
        }
    }
    Ok(hir::Program { externs, funcs })
}

impl Ast2Hir {
    fn compile_func(&mut self, mut f: ast::Function) -> Result<Vec<hir::Function>> {
        self.chapters.clear();
        self.chapters.push_back(Chapter::new());
        for (expr, _) in f.body_stmts.drain(..).collect::<Vec<_>>() {
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
        orig_func: ast::Function,
        mut chapters: VecDeque<Chapter>,
    ) -> Result<Vec<hir::Function>> {
        let n_chapters = chapters.len();
        let mut i = 0;
        let mut last_chap_result_ty = None;
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
                        hir::Param::new(last_chap_result_ty.unwrap(), "$async_result"),
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
            last_chap_result_ty = Some(chap.async_result_ty);
            split_funcs.push(new_func);
        }
        Ok(split_funcs)
    }

    fn compile_expr(&mut self, orig_func: &ast::Function, e: ast::Expr) -> Result<hir::Expr> {
        let new_e = match e {
            ast::Expr::Number(n) => hir::Expr::Number(n),
            ast::Expr::VarRef(ref name) => {
                if self.sigs.contains_key(name) {
                    hir::Expr::VarRef(name.to_string())
                } else if self.chapters.len() == 1 {
                    // The variable is just there in the first chapter
                    hir::Expr::VarRef(name.to_string())
                } else {
                    let idx = orig_func
                        .params
                        .iter()
                        .position(|x| x.name == *name)
                        .expect(&format!("unknown variable `{}'", name));
                    hir::Expr::FunCall(
                        Box::new(hir::Expr::var_ref("chiika_env_ref")),
                        vec![hir::Expr::var_ref("$env"), hir::Expr::Number(idx as i64)],
                    )
                }
            }
            ast::Expr::OpCall(op, lhs, rhs) => {
                let l = self.compile_expr(orig_func, lhs.0)?;
                let r = self.compile_expr(orig_func, rhs.0)?;
                hir::Expr::OpCall(op, Box::new(l), Box::new(r))
            }
            ast::Expr::FunCall(fexpr, arg_exprs) => {
                let mut new_args = arg_exprs
                    .into_iter()
                    .map(|(x, _)| self.compile_expr(orig_func, x))
                    .collect::<Result<Vec<_>>>()?;
                let (ast::Expr::VarRef(callee_name), _) = *fexpr else {
                    return Err(anyhow!("not a function: {:?}", fexpr));
                };
                let Some(fun_ty) = self.sigs.get(&callee_name) else {
                    return Err(anyhow!("unknown function: {:?}", callee_name));
                };
                if fun_ty.is_async {
                    new_args.insert(0, hir::Expr::var_ref("$env"));
                    new_args.insert(
                        1,
                        hir::Expr::var_ref(chapter_func_name(&orig_func.name, self.chapters.len())),
                    );
                    let cps_call =
                        hir::Expr::FunCall(Box::new(hir::Expr::VarRef(callee_name)), new_args);

                    // Change chapter here
                    let last_chapter = self.chapters.back_mut().unwrap();
                    last_chapter.stmts.push(cps_call);
                    last_chapter.async_result_ty = (*fun_ty.ret_ty).clone().into();
                    self.chapters.push_back(Chapter::new());

                    hir::Expr::VarRef("$async_result".to_string())
                } else {
                    hir::Expr::FunCall(Box::new(hir::Expr::VarRef(callee_name)), new_args)
                }
            }
            ast::Expr::Cast(_, _) => panic!("chiika-2 does not have cast operation"),
            ast::Expr::Assign(name, rhs) => {
                hir::Expr::Assign(name, Box::new(self.compile_expr(orig_func, rhs.0)?))
            }
            //ast::Expr::While(cond_expr, body_exprs) => todo!(),
            ast::Expr::Alloc(name) => hir::Expr::Alloc(name),
            ast::Expr::Return(expr) => {
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

fn prepend_async_intro(orig_func: &ast::Function, mut stmts: Vec<hir::Expr>) -> Vec<hir::Expr> {
    let push_items = vec![hir::Expr::var_ref("$cont")].into_iter().chain(
        orig_func
            .params
            .iter()
            .map(|param| hir::Expr::var_ref(&param.name)),
    );

    let mut push_calls = push_items
        .map(|arg| {
            let cast = hir::Expr::Cast(Box::new(arg), hir::Ty::Opaque);
            hir::Expr::FunCall(
                Box::new(hir::Expr::var_ref("chiika_env_push")),
                vec![hir::Expr::var_ref("$env"), cast],
            )
        })
        .collect::<Vec<_>>();
    push_calls.append(&mut stmts);
    push_calls
}

fn append_async_outro(
    orig_func: &ast::Function,
    mut stmts: Vec<hir::Expr>,
    result_ty: hir::Ty,
) -> Vec<hir::Expr> {
    let result_value = stmts.pop().unwrap();
    let n_pop = orig_func.params.len() + 1; // +1 for $cont
    let env_pop = hir::Expr::FunCall(
        Box::new(hir::Expr::var_ref("chiika_env_pop")),
        vec![hir::Expr::var_ref("$env"), hir::Expr::Number(n_pop as i64)],
    );
    let fun_ty = hir::FunTy {
        is_async: false, // chiika-1 does not have notion of asyncness
        param_tys: vec![hir::Ty::ChiikaEnv, result_ty],
        ret_ty: Box::new(hir::Ty::RustFuture),
    };
    let cast = hir::Expr::Cast(Box::new(env_pop), hir::Ty::Fun(fun_ty));
    let call_cont = hir::Expr::FunCall(
        Box::new(cast),
        vec![hir::Expr::var_ref("$env"), result_value],
    );
    stmts.push(call_cont);
    stmts
}

/// Create name of generated function like `foo_1`
fn chapter_func_name(orig_name: &str, chapter_idx: usize) -> String {
    format!("{}_{}", orig_name, chapter_idx)
}
