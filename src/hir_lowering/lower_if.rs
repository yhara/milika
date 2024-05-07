//! Converts if-else expressions to a sequence of blocks.
//! Intended to use `cf.cond_br` rather than `scf.if` which (IIRC) cannot contain
//! `func.return`.
//!
//! The branches must not contain async function calls (use lower_async_if to
//! remove them first.)
//!
//! Example:
//! ```
//! // Before
//! fun foo() {
//!   ...
//!   x = if (a) {
//!     b ...
//!     yield c
//!   } else {
//!     d ...
//!     yield e
//!   }
//!   ...
//!   x + ...
//!
//! // After
//! fun foo() -> Foo {
//!   ...
//!     cond_br a, ^bb1(), ^bb2()
//!
//!   ^bb1():
//!     b ...
//!     br ^bb3(c)
//!
//!   ^bb2():
//!     d ...
//!     br ^bb3(e)
//!
//!   ^bb3(x):
//!     ...
//!     x + ...
//! }
//! ```
use crate::hir;
use crate::hir::blocked;
use crate::hir::rewriter::HirRewriter;
use anyhow::Result;

pub fn run(program: hir::Program) -> blocked::Program {
    let funcs = program
        .funcs
        .into_iter()
        .map(|f| {
            let mut c = Compiler::new(&f);
            c.compile_func(f.body_stmts);
            blocked::Function {
                name: f.name,
                params: f.params,
                ret_ty: f.ret_ty,
                body_blocks: c.blocks,
            }
        })
        .collect();
    blocked::Program {
        externs: program.externs,
        funcs,
    }
}

struct Compiler {
    blocks: Vec<blocked::Block>,
}

impl Compiler {
    fn new(f: &hir::Function) -> Self {
        let first_block =
            blocked::Block::new_empty(f.params.iter().map(|p| p.ty.clone()).collect());
        Compiler {
            blocks: vec![first_block],
        }
    }

    fn compile_func(&mut self, body_stmts: Vec<hir::TypedExpr>) {
        let new_stmts = self.walk_exprs(body_stmts).unwrap();
        for e in new_stmts {
            self.push(e);
        }
    }

    fn push(&mut self, e: hir::TypedExpr) {
        self.blocks.last_mut().unwrap().stmts.push(e);
    }
}

impl HirRewriter for Compiler {
    fn rewrite_expr(&mut self, e: hir::TypedExpr) -> Result<hir::TypedExpr> {
        match e.0 {
            hir::Expr::If(cond, mut then_exprs, mut else_exprs) => {
                let if_ty = e.1;
                let id = self.blocks.len() - 1;
                self.push(hir::Expr::cond_br(*cond, id + 1, id + 2));

                let hir::Expr::Yield(v) = then_exprs.pop().unwrap().0 else {
                    panic!("expected yield");
                };
                then_exprs.push(hir::Expr::br(*v, id + 3));
                let then_block = blocked::Block::new(vec![], then_exprs);
                self.blocks.push(then_block);

                let hir::Expr::Yield(v) = else_exprs.pop().unwrap().0 else {
                    panic!("expected yield");
                };
                else_exprs.push(hir::Expr::br(*v, id + 3));
                let else_block = blocked::Block::new(vec![], else_exprs);
                self.blocks.push(else_block);

                let endif_block = blocked::Block::new_empty(vec![if_ty.clone()]);
                self.blocks.push(endif_block);
                Ok(hir::Expr::block_arg_ref(if_ty))
            }
            _ => Ok(e),
        }
    }
}
