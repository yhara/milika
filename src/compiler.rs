use crate::ast;
use crate::asyncness_check::gather_sigs;
use anyhow::{anyhow, Result};
use melior::{
    dialect::{self, ods::r#async, DialectRegistry},
    ir::{
        self,
        attribute::{FlatSymbolRefAttribute, IntegerAttribute, StringAttribute, TypeAttribute},
        r#type::{FunctionType, IntegerType, MemRefType, Type},
    },
    //pass::{self, PassManager},
    utility::{register_all_dialects, register_all_llvm_translations},
};
use std::collections::HashMap;

/// Get the first result value of an operation.
/// Panics if the operation yields no value
fn val<'c, 'a>(x: &'a ir::OperationRef<'c, 'a>) -> ir::Value<'c, 'a> {
    x.result(0)
        .unwrap_or_else(|_| panic!("this operation has no value"))
        .into()
}

//fn vals<'c>(xs: &'c [ir::OperationRef<'c, 'c>]) -> Vec<ir::Value<'c, 'c>> {
//    let mut v = vec![];
//    for x in xs {
//        v.push(val(x));
//    }
//    v
//}

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

impl<'run: 'c, 'c> Compiler<'run, 'c> {
    fn compile_program(&mut self, ast: ast::Program) -> Result<()> {
        let module = ir::Module::new(self.unknown_location());
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

        //module.as_operation().dump();
        //println!("--");
        //assert!(module.as_operation().verify());

        // Convert to LLVM Dialect
        //let pass_manager = PassManager::new(&self.context);
        //pass_manager.add_pass(pass::r#async::create_async_func_to_async_runtime());
        //pass_manager.add_pass(pass::r#async::create_async_to_async_runtime());
        //pass_manager.add_pass(pass::conversion::create_async_to_llvm());
        //pass_manager.add_pass(pass::conversion::create_func_to_llvm());
        //pass_manager
        //    .nested_under("func.func")
        //    .add_pass(pass::conversion::create_arith_to_llvm());
        //pass_manager
        //    .nested_under("func.func")
        //    .add_pass(pass::conversion::create_index_to_llvm());
        //pass_manager.add_pass(pass::conversion::create_scf_to_control_flow());
        //pass_manager.add_pass(pass::conversion::create_control_flow_to_llvm());
        //pass_manager.add_pass(pass::conversion::create_finalize_mem_ref_to_llvm());
        //pass_manager.run(&mut module).unwrap();
        eprintln!("--CUTHERE--");
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
            TypeAttribute::new(self.function_type(&ext.fun_ty())?.into()),
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
            self.compile_expr(&block, stmt)?;
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
            TypeAttribute::new(self.function_type(&f.fun_ty(false))?.into()),
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

    fn compile_expr(
        &'run self,
        block: &'c ir::Block,
        expr: &ast::Expr,
    ) -> Result<ir::OperationRef<'c, 'c>> {
        let op = match expr {
            ast::Expr::Number(n) => self.const_int(*n),
            ast::Expr::VarRef(name) => return self.compile_varref(block, name),
            ast::Expr::OpCall(op, lhs, rhs) => return self.compile_op_call(block, op, lhs, rhs),
            ast::Expr::FunCall(fexpr, arg_exprs) => {
                return self.compile_funcall(block, fexpr, arg_exprs)
            }
            ast::Expr::If(cond, then, els) => return self.compile_if(block, cond, then, els),
            ast::Expr::Alloc(name) => return self.compile_alloc(block, name),
            ast::Expr::Return(val_expr) => {
                let v = self.compile_expr(block, val_expr)?;
                dialect::func::r#return(&[val(&v)], self.unknown_location())
            }
            ast::Expr::Para(exprs) => {
                let block = ir::Block::new(&[]);
                for expr in exprs {
                    self.compile_expr(&block, expr)?;
                }
                let zero = self.compile_number(&block, 0);
                block.append_operation(
                    r#async::YieldOp::builder(&self.context, self.unknown_location())
                        .operands(&[val(&zero)])
                        .build()
                        .into(),
                );
                let region = ir::Region::new();
                region.append_block(block);
                r#async::ExecuteOp::builder(&self.context, self.unknown_location())
                    .token(Type::parse(&self.context, "!async.token").unwrap())
                    .body_results(&[Type::parse(&self.context, "!async.value<i64>").unwrap()])
                    .dependencies(&[])
                    .body_operands(&[])
                    .body_region(region)
                    .build()
                    .into()
            }
            _ => todo!("{:?}", expr),
        };
        Ok(block.append_operation(op))
    }

    fn compile_op_call(
        &'run self,
        block: &'c ir::Block,
        operator: &str,
        lhs: &ast::Expr,
        rhs: &ast::Expr,
    ) -> Result<ir::OperationRef<'c, 'c>> {
        let f = match operator {
            "+" => dialect::arith::addi,
            "-" => dialect::arith::subi,
            "*" => dialect::arith::muli,
            "/" => dialect::arith::divsi,
            _ => return self.compile_cmp(block, operator, lhs, rhs),
        };
        let op = f(
            val(&self.compile_expr(block, lhs)?),
            val(&self.compile_expr(block, rhs)?),
            self.unknown_location(),
        );
        Ok(block.append_operation(op))
    }

    fn compile_cmp(
        &'run self,
        block: &'c ir::Block,
        operator: &str,
        lhs: &ast::Expr,
        rhs: &ast::Expr,
    ) -> Result<ir::OperationRef<'c, 'c>> {
        let pred = match operator {
            "==" => dialect::arith::CmpiPredicate::Eq,
            "!=" => dialect::arith::CmpiPredicate::Ne,
            "<" => dialect::arith::CmpiPredicate::Ult,
            "<=" => dialect::arith::CmpiPredicate::Ule,
            ">" => dialect::arith::CmpiPredicate::Ugt,
            ">=" => dialect::arith::CmpiPredicate::Uge,
            _ => panic!("unkown operator"),
        };
        let op = dialect::arith::cmpi(
            &self.context,
            pred,
            val(&self.compile_expr(block, lhs)?),
            val(&self.compile_expr(block, rhs)?),
            self.unknown_location(),
        );
        Ok(block.append_operation(op))
    }

    fn compile_funcall(
        &'run self,
        block: &'c ir::Block,
        fexpr: &ast::Expr,
        arg_exprs: &[ast::Expr],
    ) -> Result<ir::OperationRef<'c, 'c>> {
        let fun_ty = match fexpr {
            ast::Expr::VarRef(s) => {
                if let Some(t) = self.sigs.get(s) {
                    t
                } else {
                    todo!()
                }
            }
            _ => return Err(anyhow!("not a function?")),
        };

        let f = self.compile_expr(block, fexpr)?;

        let mut args = vec![];
        for e in arg_exprs {
            args.push(self.compile_expr(block, e)?.result(0)?.into());
        }

        let result_types = fun_ty
            .param_tys
            .iter()
            .map(|t| self.mlir_type(t))
            .collect::<Result<Vec<_>>>()?;
        let op =
            dialect::func::call_indirect(val(&f), &args, &result_types, self.unknown_location());
        Ok(block.append_operation(op))
    }

    fn compile_if(
        &'run self,
        block: &'c ir::Block,
        cond_expr: &ast::Expr,
        then: &[ast::Expr],
        els: &Option<Vec<ast::Expr>>,
    ) -> Result<ir::OperationRef<'c, 'c>> {
        let cond_result = self.compile_expr(block, cond_expr)?;
        let then_region = self.compile_exprs(then, true)?;
        let else_region = self.compile_exprs(if let Some(v) = els { v } else { &[] }, true)?;
        let op = dialect::scf::r#if(
            val(&cond_result),
            Default::default(),
            then_region,
            else_region,
            self.unknown_location(),
        );
        Ok(block.append_operation(op))
    }

    fn compile_alloc(
        &'run self,
        block: &'c ir::Block,
        name: &str,
    ) -> Result<ir::OperationRef<'c, 'c>> {
        let op = dialect::memref::alloca(
            &self.context,
            MemRefType::new(self.i64_type().into(), &[], None, None),
            &[],
            &[],
            None,
            self.unknown_location(),
        );
        Ok(block.append_operation(op))
    }

    fn compile_varref(
        &'run self,
        block: &'c ir::Block,
        name: &str,
    ) -> Result<ir::OperationRef<'c, 'c>> {
        let op = if let Some(fun_ty) = self.sigs.get(name) {
            dialect::func::constant(
                &self.context,
                FlatSymbolRefAttribute::new(self.context, name),
                self.function_type(fun_ty)?,
                self.unknown_location(),
            )
        } else {
            todo!()
        };
        Ok(block.append_operation(op))
    }

    fn compile_number(&'run self, block: &'c ir::Block, n: i64) -> ir::OperationRef<'c, 'c> {
        block.append_operation(self.const_int(n))
    }

    /// Returns a newly created region that contains `exprs`.
    fn compile_exprs(&'run self, exprs: &[ast::Expr], terminate: bool) -> Result<ir::Region> {
        let block = ir::Block::new(&[]);
        for expr in exprs {
            self.compile_expr(&block, expr)?;
        }
        if terminate {
            let op = dialect::scf::r#yield(&[], self.unknown_location());
            block.append_operation(op);
        }
        let region = ir::Region::new();
        region.append_block(block);
        Ok(region)
    }

    fn const_int(&'run self, n: i64) -> ir::Operation<'c> {
        dialect::arith::constant(
            &self.context,
            IntegerAttribute::new(n, IntegerType::new(&self.context, 64).into()).into(),
            self.unknown_location(),
        )
    }

    fn function_type(&self, fun_ty: &ast::FunTy) -> Result<ir::r#type::FunctionType> {
        let param_tys = self.mlir_types(&fun_ty.param_tys)?;
        let ret_ty = self.mlir_type(&fun_ty.ret_ty)?;
        Ok(FunctionType::new(&self.context, &param_tys, &[ret_ty]).into())
    }

    fn mlir_types(&self, tys: &[ast::Ty]) -> Result<Vec<ir::Type>> {
        tys.iter().map(|x| self.mlir_type(x)).collect()
    }

    fn mlir_type(&self, ty: &ast::Ty) -> Result<ir::Type> {
        let t = match ty {
            ast::Ty::Raw(s) => match &s[..] {
                "none" => Type::none(&self.context).into(),
                "int" => self.i64_type().into(),
                _ => return Err(anyhow!("unknown type `{}'", s)),
            },
            ast::Ty::Fun(fun_ty) => self.function_type(fun_ty)?.into(),
        };
        Ok(t)
    }

    fn i64_type(&self) -> ir::r#type::IntegerType {
        ir::r#type::IntegerType::new(&self.context, 64)
    }

    fn identifier(&self, s: &str) -> ir::Identifier {
        ir::Identifier::new(&self.context, s)
    }

    fn str_attr(&self, s: &str) -> StringAttribute {
        StringAttribute::new(&self.context, s)
    }

    fn loc(&self, span: &ast::Span) -> ir::Location {
        //        let mut line = 1;
        //        let mut col = 1;
        //        for i in 0..span.start {
        //            match self.src.as_bytes().get(i).unwrap() {
        //                b'\n' => {
        //                    line += 1;
        //                    col = 1;
        //                }
        //                _ => {
        //                    col += 1;
        //                }
        //            }
        //        }
        ir::Location::new(
            &self.context,
            &self.filename,
            span.location_line() as usize,
            span.get_utf8_column(),
        )
    }

    fn unknown_location(&self) -> ir::Location {
        ir::Location::unknown(&self.context)
    }
}
