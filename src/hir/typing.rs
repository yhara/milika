use crate::ast;
use crate::hir;
use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::collections::VecDeque;

struct Typing {
    /// Functions whose signatures are already known
    resolved_funcs: HashMap<String, hir::FunTy>,
    /// Functions whose asyncness depends on other functions
    pending_funcs: HashMap<(String, Vec<String>)>,
    /// Stack to prevent infinite recursion
    compiling_functions: Vec<String>,
}

pub fn run(hir: hir::Program) -> Result<hir::Program> {
    let mut c = Typing {
        resolved_funcs: HashMap::new(),
        pending_funcs: HashMap::new(),
        compiling_functions: vec![],
    };

    for e in hir.externs {
        c.resolved_funcs.insert(e.name.clone(), e.fun_ty());
    }

    let mut funcs = HashMap::new();
    for f in hir.funcs {
        c.compiling_functions.clear();
        let new_f = c.compile_func(f)?;
        funcs.insert(new_f.name.clone(), new_f);
    }

    let pending_funcs = VecDeque::from_iter(c.pending_funcs);
    while let Some((name, deps)) = pending_funcs.pop_front() {
        resolve_pending_asyncness(
            &mut funcs,
            &c.resolved_funcs,
            &mut pending_funcs,
            name,
            deps,
        );
    }

    Ok(hir::Program { funcs, ..hir })
}

fn resolve_pending_asyncness(
    funcs: &mut HashMap<String, hir::Function>,
    resolved_funcs: &HashMap<String, hir::FunTy>,
    pending_funcs: &mut VecDeque<(String, Vec<String>)>,
    name: String,
    deps: Vec<String>,
) {
    let new_deps = HashSet::from_iter(deps.iter().cloned());
    for dep in deps {
        if let Some(dep_ty) = resolved_funcs.get(&dep) {
            if dep_ty.ret_ty.is_async() {
                // It is async if any of the dependencies are async
                let f = funcs.get_mut(&name).unwrap();
                f.ret_ty = Box::new(hir::FunTy::Async(f.ret_ty));
                resolved_funcs.insert(name.clone(), f.fun_ty());
            } else {
                new_deps.remove(&dep);
            }
        } else {
            pending_funcs.push_back((name, new_deps.into_iter().collect()));
            return;
        }
    }
    // None of the dependencies are async
    let f = funcs.get(&name).unwrap();
    resolved_funcs.insert(name.clone(), f.fun_ty());
}

impl Typing {
    fn compile_func(&mut self, f: hir::Function) -> Result<hir::Function> {
        self.compiling_functions.push(f.name.clone());
        let mut lvars = HashMap::new();
        f.body_stmts = f
            .body_stmts
            .drain(..)
            .map(|e| self.compile_expr(&f, &mut lvars, e))
            .collect::<Result<Vec<_>>>()?;
        Ok(f)
    }

    fn compile_expr(
        &mut self,
        orig_func: &hir::Function,
        lvars: &mut HashMap<String, hir::Ty>,
        e: hir::TypedExpr,
    ) -> Result<()> {
        let new_e = match e.0 {
            (hir::Expr::Number(_), _) => (e.0, hir::Ty::Int),
            (hir::Expr::PseudoVar(hir::PseudoVar::True), _) => (e.0, hir::Ty::Bool),
            (hir::Expr::PseudoVar(hir::PseudoVar::False), _) => (e.0, hir::Ty::Bool),
            (hir::Expr::PseudoVar(hir::PseudoVar::Null), _) => (e.0, hir::Ty::Null),
            (hir::Expr::LVarRef(name), _) => {
                if let Some(ty) = lvars.get(name) {
                    (e.0, ty.clone())
                } else {
                    return Err(anyhow!("[BUG] unknown variable `{name}'"));
                }
            }
            (hir::Expr::ArgRef(i), _) => (e.0, orig_func.params[i].ty.clone()),
            (hir::Expr::FuncRef(name), _) => {
                if let Some(fun_ty) = self.resolved_funcs.get(name) {
                    (e.0, hir::Ty::Fun(fun_ty.clone()))
                } else {
                    (e.0, hir::Ty::TyOfFun(name.clone()))
                }
            }
            (hir::Expr::OpCall(op, l, r), _) => {
                let l = self.compile_expr(orig_func, lvars, l)?;
                let r = self.compile_expr(orig_func, lvars, r)?;
                hir::Expr::op_call(op, l, r)?
            }
            (hir::Expr::FunCall(fexpr, args), _) => {
                let new_fexpr = self.compile_expr(orig_func, lvars, fexpr)?;
                self.compile_fun_call(orig_func, lvars, new_fexpr, args)?
            }
            (hir::Expr::If(cond, then, els), _) => {
                let cond = self.compile_expr(orig_func, lvars, cond)?;
                let then = self.compile_exprs(orig_func, lvars, then)?;
                let els = self.compile_exprs(orig_func, lvars, els)?;
                hir::Expr::if_(cond, then, els)?
            }
            (hir::Expr::ValuedIf(cond, then, els), _) => {
                let cond = self.compile_expr(orig_func, lvars, cond)?;
                let then = self.compile_expr(orig_func, lvars, then)?;
                let els = self.compile_expr(orig_func, lvars, els)?;
                hir::Expr::valued_if(cond, then, els)?
            }
            (hir::Expr::Yield(val), _) => {
                let val = self.compile_expr(orig_func, lvars, val)?;
                hir::Expr::yield_(val)
            }
            (hir::Expr::While(cond, body), _) => {
                let cond = self.compile_expr(orig_func, lvars, cond)?;
                let body = self.compile_exprs(orig_func, lvars, body)?;
                hir::Expr::while_(cond, body)?
            }
            (hir::Expr::Alloc(name), _) => {
                // Milika vars are always Int now
                lvars.insert(name.clone(), hir::Ty::Int);
                (e.0, hir::Ty::Void)
            }
            (hir::Expr::Assign(name, val), _) => {
                let val = self.compile_expr(orig_func, lvars, val)?;
                hir::Expr::assign(name, val)
            }
            (hir::Expr::Return(val), _) => {
                let val = self.compile_expr(orig_func, lvars, val)?;
                if val.1 != *orig_func.ret_ty {
                    return Err(anyhow!(
                        "return type mismatch: {} should return {:?} but got {:?}",
                        orig_func.name,
                        orig_func.ret_ty,
                        val.1
                    ));
                }
                hir::Expr::return_(val)
            }
            (hir::Expr::Cast(val, ty), _) => {
                return Err(anyhow!("[BUG] Cast unexpected here"));
            }
        };
        Ok(new_e)
    }

    fn compile_exprs(
        &self,
        orig_func: &hir::Function,
        lvars: &mut HashMap<String, hir::Ty>,
        es: &[hir::TypedExpr],
    ) -> Result<Vec<hir::TypedExpr>> {
        es.iter()
            .map(|e| self.compile_expr(orig_func, lvars, &e))
            .collect()
    }

    fn compile_fun_call(
        &mut self,
        orig_func: &hir::Function,
        lvars: &mut HashMap<String, hir::Ty>,
        fexpr: hir::TypedExpr,
        args: Vec<hir::TypedExpr>,
    ) -> Result<(hir::Expr, hir::Ty)> {
        let new_expr = self.compile_expr(orig_func, lvars, fexpr)?;
        let new_args = self.compile_exprs(orig_func, lvars, args)?;
        loop {
            let fun_ty = if let Some(fun_ty) = self.resolved_funcs.get(name) {
                fun_ty.clone()
            } else if let Some(deps) = self.pending_funcs.get(name) {
            } else if self.compiling_functions.contains(name) {
                let f = orig_func.clone();
                self.pending_funcs.push((f.name.clone(), f.deps()));
                hir::FunTy::TyOfFun(name.clone())
            } else {
                self.compiling_functions.push(name.clone());
                self.compile_func(f)?;
                continue;
            };
            break;
        }
        check_funcall_arg_types(&fun_ty.params, &args)?;
        Ok(hir::Expr::fun_call(new_fexpr, args))
    }
}

fn check_funcall_arg_types(param_tys: &[hir::Ty], args: &[(hir::Expr, hir::Ty)]) -> Result<()> {
    for (param_ty, (_, arg_ty)) in param_tys.iter().zip(args.iter()) {
        if param_ty != arg_ty {
            return Err(anyhow!(
                "funcall arg type mismatch: expected {:?} but got {:?}",
                param_ty,
                arg_ty
            ));
        }
    }
    Ok(())
}
