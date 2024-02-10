use crate::ast;
use crate::asyncness_check::gather_sigs;
use anyhow::{anyhow, Result};
use melior::{
    dialect::{self, DialectRegistry},
    ir::{
        self,
        attribute::{IntegerAttribute, StringAttribute, TypeAttribute},
        r#type::{FunctionType, IntegerType, Type},
    },
    pass::{self, PassManager},
    utility::{register_all_dialects, register_all_llvm_translations},
};
use std::collections::HashMap;

/// Get the first result value of an operation.
/// Panics if the operation yields no value
fn val<'c, 'a>(x: &'a ir::Operation<'c>) -> ir::Value<'c, 'a> {
    x.result(0)
        .unwrap_or_else(|_| panic!("this operation has no value"))
        .into()
}

struct Compiler<'run: 'c, 'c> {
    filename: &'run str,
    src: &'run str,
    context: &'c melior::Context,
    sigs: HashMap<String, ast::FunTy>,
}

pub fn run(filename: &str, src: &str, ast: ast::Program) -> Result<()> {
    let registry = DialectRegistry::new();
    register_all_dialects(&registry);

    let context = melior::Context::new();
    context.append_dialect_registry(&registry);
    context.load_all_available_dialects();
    register_all_llvm_translations(&context);

    let mut c = Compiler {
        filename,
        src,
        context: &context,
        sigs: gather_sigs(&ast)?,
    };
    c.compile_program(ast)?;
    Ok(())
}

impl<'run, 'c> Compiler<'run, 'c> {
    fn compile_program(&mut self, ast: ast::Program) -> Result<()> {
        let mut module = ir::Module::new(self.unknown_location());
        let block = module.body();

        for decl in ast {
            match decl {
                ast::Declaration::Extern(e) => {
                    block.append_operation(self.compile_extern(e.0, e.1)?);
                }
                ast::Declaration::Function(f) => {
                    block.append_operation(self.compile_func(f.0, f.1, true)?);
                }
            }
        }

        module.as_operation().dump();
        //assert!(module.as_operation().verify());
        println!("--");

        // Convert to LLVM Dialect
        let pass_manager = PassManager::new(&self.context);
        pass_manager.add_pass(pass::conversion::create_func_to_llvm());
        pass_manager
            .nested_under("func.func")
            .add_pass(pass::conversion::create_arith_to_llvm());
        pass_manager
            .nested_under("func.func")
            .add_pass(pass::conversion::create_index_to_llvm());
        pass_manager.add_pass(pass::conversion::create_scf_to_control_flow());
        pass_manager.add_pass(pass::conversion::create_control_flow_to_llvm());
        pass_manager.add_pass(pass::conversion::create_finalize_mem_ref_to_llvm());
        pass_manager.run(&mut module).unwrap();
        module.as_operation().dump();
        assert!(module.as_operation().verify());
        Ok(())
    }

    fn compile_extern(&self, ext: ast::Extern, span: ast::Span) -> Result<ir::Operation> {
        let attrs = vec![(
            self.identifier("sym_visibility"),
            self.str_attr("private").into(),
        )];
        Ok(dialect::func::func(
            &self.context,
            self.str_attr(&ext.name),
            TypeAttribute::new(self.function_type(&ext.fun_ty())?),
            Default::default(),
            &attrs,
            self.loc(&span),
        ))
    }

    fn compile_func(
        &self,
        f: ast::Function,
        span: ast::Span,
        is_extern: bool,
    ) -> Result<ir::Operation> {
        let block = self.create_main_block(&f)?;
        for stmt in &f.body_stmts {
            block.append_operation(self.compile_expr(&block, stmt)?);
        }

        let region = ir::Region::new();
        region.append_block(block);

        let attrs = if is_extern {
            vec![(
                self.identifier("sym_visibility"),
                self.str_attr("private").into(),
            )]
        } else {
            vec![]
        };
        Ok(dialect::func::func(
            &self.context,
            self.str_attr(&f.name),
            TypeAttribute::new(self.function_type(&f.fun_ty(false))?),
            region,
            &attrs,
            self.loc(&span),
        ))
    }

    fn create_main_block(&self, f: &ast::Function) -> Result<ir::Block> {
        let param_tys = f
            .params
            .iter()
            .map(|x| self.mlir_type(&x.ty))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .map(|x| (x, self.unknown_location()))
            .collect::<Vec<_>>();
        Ok(ir::Block::new(&param_tys))
    }

    //    fn compile_stmt(&self, block: &ir::Block, expr: &ast::Expr) -> Result<ir::Operation> {
    //        self.compile_expr(block, expr)
    //    }

    fn compile_expr<'a: 'run>(
        &'run self,
        block: &'a ir::Block,
        expr: &ast::Expr,
    ) -> Result<ir::Operation<'run>> {
        let op = match expr {
            ast::Expr::Number(n) => dialect::arith::constant(
                &self.context,
                IntegerAttribute::new(*n, IntegerType::signed(&self.context, 64).into()).into(),
                self.unknown_location(),
            ),
            ast::Expr::Return(val_expr) => {
                let v = block.append_operation(self.compile_expr(block, val_expr)?);
                dialect::func::r#return(&[val(&v)], self.unknown_location())
            }
            _ => todo!("{:?}", expr),
        };
        Ok(op)
    }

    fn function_type(&self, fun_ty: &ast::FunTy) -> Result<ir::Type> {
        let param_tys = fun_ty
            .param_tys
            .iter()
            .map(|x| self.mlir_type(x))
            .collect::<Result<Vec<_>>>()?;
        let ret_ty = self.mlir_type(&fun_ty.ret_ty)?;
        Ok(FunctionType::new(&self.context, &param_tys, &[ret_ty]).into())
    }

    fn mlir_type(&self, ty: &ast::Ty) -> Result<ir::Type> {
        let t = match ty {
            ast::Ty::Raw(s) => match &s[..] {
                "none" => Type::none(&self.context).into(),
                "int" => ir::r#type::IntegerType::signed(&self.context, 64).into(),
                _ => return Err(anyhow!("unknown type `{}'", s)),
            },
            ast::Ty::Fun(fun_ty) => return self.function_type(fun_ty),
        };
        Ok(t)
    }

    fn identifier(&self, s: &str) -> ir::Identifier {
        ir::Identifier::new(&self.context, s)
    }

    fn str_attr(&self, s: &str) -> StringAttribute {
        StringAttribute::new(&self.context, s)
    }

    fn loc(&self, span: &ast::Span) -> ir::Location {
        let mut line = 1;
        let mut col = 1;
        for i in 0..span.start {
            match self.src.as_bytes().get(i).unwrap() {
                b'\n' => {
                    line += 1;
                    col = 1;
                }
                _ => {
                    col += 1;
                }
            }
        }
        ir::Location::new(&self.context, &self.filename, line, col)
    }

    fn unknown_location(&self) -> ir::Location {
        ir::Location::unknown(&self.context)
    }
}
