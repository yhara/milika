use crate::ast;
use crate::asyncness_check::gather_sigs;
use anyhow::Result;
use melior::{
    dialect::{self, DialectRegistry},
    ir::{
        attribute::{StringAttribute, TypeAttribute},
        r#type::FunctionType,
        Block, Identifier, Location, Module, Operation, Region, Type,
    },
    pass::{self, PassManager},
    utility::{register_all_dialects, register_all_llvm_translations},
    Context,
};
use std::collections::HashMap;

struct Compiler {
    context: Context,
    sigs: HashMap<String, ast::FunTy>,
}

pub fn run(ast: ast::Program) -> Result<()> {
    let registry = DialectRegistry::new();
    register_all_dialects(&registry);

    let context = Context::new();
    context.append_dialect_registry(&registry);
    context.load_all_available_dialects();
    register_all_llvm_translations(&context);

    let mut c = Compiler {
        context,
        sigs: gather_sigs(&ast)?,
    };
    c.compile_program(ast)?;
    Ok(())
}

impl Compiler {
    fn compile_program(&mut self, ast: ast::Program) -> Result<()> {
        let mut module = Module::new(self.location());

        for decl in ast {
            match decl {
                ast::Declaration::Extern(e) => {
                    module.body().append_operation(self.compile_extern(e)?);
                }
                ast::Declaration::Function(f) => {
                    module.body().append_operation(self.compile_func(f)?);
                }
            }
        }

        module.as_operation().dump();
        assert!(module.as_operation().verify());
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
        assert!(module.as_operation().verify());
        module.as_operation().dump();
        Ok(())
    }

    fn compile_extern(&self, ext: ast::Extern) -> Result<Operation> {
        self.compile_func(ext.into_empty_func())
    }

    fn compile_func(&self, func: ast::Function) -> Result<Operation> {
        let index_type = Type::index(&self.context);
        let block = Block::new(&[(index_type, self.location()), (index_type, self.location())]);
        let sum = block.append_operation(dialect::arith::addi(
            block.argument(0).unwrap().into(),
            block.argument(1).unwrap().into(),
            self.location(),
        ));

        block.append_operation(dialect::func::r#return(
            &[sum.result(0).unwrap().into()],
            self.location(),
        ));

        let region = Region::new();
        region.append_block(block);

        Ok(dialect::func::func(
            &self.context,
            self.str_attr(&func.name),
            TypeAttribute::new(
                FunctionType::new(&self.context, &[index_type, index_type], &[index_type]).into(),
            ),
            region,
            &[],
            self.location(),
        ))
    }

    //    fn compile_expr(&self, expr: ast::Expr) -> todo {
    //        match expr {
    //            ast::Expr::Number(n) => {}
    //            _ => todo!(),
    //        }
    //    }

    fn identifier(&self, s: &str) -> Identifier {
        Identifier::new(&self.context, s)
    }

    fn str_attr(&self, s: &str) -> StringAttribute {
        StringAttribute::new(&self.context, s)
    }

    fn location(&self) -> Location {
        Location::unknown(&self.context)
    }
}
