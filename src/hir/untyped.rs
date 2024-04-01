use crate::ast;
use crate::hir;
use anyhow::{anyhow, Result};
use std::collections::HashSet;

/// Create untyped HIR (i.e. contains Ty::Unknown) from AST
pub fn create(ast: &ast::Program) -> hir::Program {
    let func_names = ast
        .externs
        .iter()
        .map(|e| e.name.clone())
        .chain(ast.funcs.iter().map(|f| f.name.clone()))
        .collect::<HashSet<_>>();

    let mut externs = vec![];
    for e in &ast.externs {
        externs.push(compile_extern(e));
    }

    let c = Compiler { func_names };
    let mut funcs = vec![];
    for f in &ast.funcs {
        funcs.push(c.compile_func(f));
    }
    hir::Program { externs, funcs }
}

struct Compiler {
    func_names: HashSet<String>,
}

impl Compiler {
    fn compile_func(&self, f: &ast::Function) -> Result<hir::Function> {
        let mut params = vec![];
        for p in &f.params {
            params.push(hir::Param {
                name: p.name.clone(),
                ty: compile_ty(p.ty)?,
            });
        }
        let t = compile_ty(&f.ret_ty)?;
        let mut lvars = HashSet::new();
        Ok(hir::Function {
            name: f.name.clone(),
            params,
            ret_ty: if t.is_async {
                hir::Ty::Async(Box::new(t))
            } else {
                t
            },
            body_stmts: f
                .body_stmts
                .iter()
                .map(|e| compile_expr(&f, &mut lvars, &e))
                .collect::<Result<Vec<_>>>()?,
        })
    }

    fn compile_expr(
        &self,
        f: &ast::Function,
        lvars: &mut HashSet<String>,
        x: &ast::Expr,
    ) -> Result<hir::Expr> {
        let e = match x {
            ast::Expr::Number(i) => hir::Expr::Number(*i),
            ast::Expr::VarRef(name) => self.compile_var_ref(f, lvars, name)?,
            ast::Expr::OpCall(op, lhs, rhs) => {
                let lhs = compile_expr(f, lvars, lhs)?;
                let rhs = compile_expr(f, lvars, rhs)?;
                hir::Expr::OpCall(op.clone(), Box::new(lhs), Box::new(rhs))
            }
            ast::Expr::FunCall(fexpr, args) => {
                let fexpr = compile_expr(f, lvars, fexpr)?;
                let mut args = vec![];
                for a in args {
                    args.push(compile_expr(f, lvars, a)?);
                }
                hir::Expr::FunCall(Box::new(fexpr), args)
            }
            ast::Expr::If(cond, then, els) => {
                let cond = compile_expr(f, lvars, &cond)?;
                let then = compile_exprs(f, lvars, &then)?;
                let els = if let Some(els) = &els {
                    compile_exprs(f, lvars, &els)?;
                } else {
                    vec![]
                };
                hir::Expr::If(cond, then, els)
            }
            ast::Expr::While(cond, body) => {
                let cond = compile_expr(f, lvars, &cond)?;
                let body = compile_exprs(f, lvars, &body)?;
                hir::Expr::While(cond, body)
            }
            ast::Expr::Alloc(name) => {
                lvars.insert(name.clone());
                hir::Expr::Alloc(name.clone())
            }
            ast::Expr::Assign(name, rhs) => {
                let rhs = compile_expr(f, lvars, &rhs)?;
                hir::Expr::Assign(name.clone(), Box::new(rhs))
            }
            ast::Expr::Return(v) => {
                let e = compile_expr(f, lvars, v)?;
                hir::Expr::Return(Box::new(e))
            }
        };
        Ok((e, hir::Ty::Unknown))
    }

    fn compile_var_ref(
        &self,
        f: &ast::Function,
        lvars: &mut HashSet<String>,
        name: &str,
    ) -> Result<hir::Expr> {
        if lvars.contains(name) {
            Ok(hir::Expr::LVarRef(name.to_string()))
        } else if let Some(idx) = f.params.iter().position(|p| p.name == name) {
            Ok(hir::Expr::ArgRef(idx))
        } else if self.func_names.contains(name) {
            Ok(hir::Expr::FuncRef(name.to_string()))
        } else {
            Err(anyhow!("unknown variable: {name}"))
        }
    }

    fn compile_exprs(
        &self,
        f: &ast::Function,
        lvars: &mut Vec<(String, hir::Ty)>,
        xs: &[ast::Expr],
    ) -> Result<Vec<hir::Expr>> {
        let mut es = vec![];
        for x in xs {
            es.push(compile_expr(f, lvars, x)?);
        }
        Ok(es)
    }
}

fn compile_extern(e: &ast::Extern) -> Result<hir::Extern> {
    let mut params = vec![];
    for p in &e.params {
        params.push(hir::Param {
            name: p.name.clone(),
            ty: compile_ty(p.ty)?,
        });
    }
    let t = compile_ty(&e.ret_ty)?;
    Ok(hir::Extern {
        is_internal: e.is_internal,
        name: e.name.clone(),
        params,
        ret_ty: if t.is_async {
            hir::Ty::Async(Box::new(t))
        } else {
            t
        },
    })
}

fn compile_ty(x: ast::Ty) -> Result<hir::Ty> {
    let t = match x {
        ast::Ty::Raw(s) => match &s[..] {
            "Null" => hir::Ty::Null,
            "Int" => hir::Ty::Int,
            "Bool" => hir::Ty::Bool,
            // Internally used types (in src/prelude.rs)
            "ANY" => hir::Ty::Any,
            "ENV" => hir::Ty::ChiikaEnv,
            "FUTURE" => hir::Ty::RustFuture,
            _ => return Err(anyhow!("unknown type: {s}")),
        },
        ast::Ty::Fun(f) => hir::Ty::Fun(compile_fun_ty(f)?),
    };
    Ok(t)
}

fn compile_fun_ty(x: ast::FunTy) -> Result<hir::FunTy> {
    let mut params = vec![];
    for p in &x.params {
        params.push(compile_ty(p)?);
    }
    let ret_ty = Box::new(cerate_ty(x.ret_ty)?);
    Ok(hir::FunTy { params, ret_ty })
}
