use crate::hir;
use crate::hir::blocked;
use crate::hir::rewriter::HirRewriter;
use anyhow::Result;

pub fn run(program: hir::Program) -> blocked::Program {
    let funcs = program
        .funcs
        .into_iter()
        .map(|f| {
            let c = Compiler::new();
            c.compile_func(f.body_stmts);
            blocked::Function {
                is_async: f.is_async,
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
    fn new() -> Self {
        let block = vec![];
        Compiler {
            blocks: vec![block],
        }
    }

    fn compile_func(&self, body_stmts: Vec<hir::TypedExpr>) {
        let new_stmts = self.walk_exprs(body_stmts).unwrap();
        for e in new_stmts {
            self.push(e);
        }
    }

    fn push(&mut self, e: hir::TypedExpr) {
        self.blocks.last_mut().unwrap().push(e);
    }
}

impl HirRewriter for Compiler {
    fn rewrite_expr(&mut self, e: hir::TypedExpr) -> Result<hir::TypedExpr> {
        match e.0 {
            hir::Expr::If(cond, mut then_block, mut else_block) => {
                let if_ty = e.1;
                let id = self.blocks.len() - 1;
                self.push(hir::Expr::cond_br(*cond, id + 1, id + 2));
                then_block.push(hir::Expr::br(id + 2));
                self.blocks.push(then_block);
                else_block.push(hir::Expr::br(id + 2));
                self.blocks.push(then_block);
                let endif_block = vec![];
                self.blocks.push(endif_block);
                Ok(hir::Expr::block_arg_ref(if_ty))
            }
            _ => Ok(e),
        }
    }
}
