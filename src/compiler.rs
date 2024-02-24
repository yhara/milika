use crate::ast;
use crate::asyncness_check::gather_sigs;
use anyhow::{anyhow, Result};
use melior::{
    dialect::{
        self,
        //ods::r#async,
        DialectRegistry,
    },
    ir::{
        self,
        attribute::{FlatSymbolRefAttribute, IntegerAttribute, StringAttribute, TypeAttribute},
        r#type::{FunctionType, MemRefType, Type},
    },
    //pass::{self, PassManager},
    utility::{register_all_dialects, register_all_llvm_translations},
};
use std::collections::HashMap;
use train_map::TrainMap;

/// Get the first result value of an operation.
/// Panics if the operation yields no value
fn val<'c, 'a>(x: ir::OperationRef<'c, 'a>) -> ir::Value<'c, 'a> {
    x.result(0)
        .unwrap_or_else(|e| panic!("this operation has no value: {e}"))
        .into()
}

//fn vals<'c>(xs: &'c [ir::OperationRef<'c, 'a>]) -> Vec<ir::Value<'c, 'a>> {
//    let mut v = vec![];
//    for x in xs {
//        v.push(val(x));
//    }
//    v
//}

struct Compiler<'c> {
    filename: &'c str,
    src: &'c str,
    context: &'c melior::Context,
    sigs: HashMap<String, ast::FunTy>,
}

pub fn run(filename: &str, src: &str, prog: ast::Program) -> Result<()> {
    let registry = DialectRegistry::new();
    register_all_dialects(&registry);

    let context = melior::Context::new();
    context.append_dialect_registry(&registry);
    context.load_all_available_dialects();
    register_all_llvm_translations(&context);

    let c = Compiler {
        filename,
        src,
        context: &context,
        sigs: gather_sigs(&prog.0)?,
    };
    c.compile_program(prog)?;
    Ok(())
}

impl<'c> Compiler<'c> {
    fn compile_program(&self, prog: ast::Program) -> Result<()> {
        let (ast, pos) = prog;
        let module = ir::Module::new(self.loc(&pos));
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

    fn compile_extern(&self, ext: ast::Extern, pos: ast::Span) -> Result<ir::Operation> {
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
            self.loc(&pos),
        ))
    }

    fn compile_func(
        &self,
        f: ast::Function,
        span: ast::Span,
        is_extern: bool,
    ) -> Result<ir::Operation<'c>> {
        let mut lvars = TrainMap::new();
        let block = self.create_main_block(&f)?;
        for stmt in &f.body_stmts {
            self.compile_expr(&block, &mut lvars, stmt)?;
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

    fn create_main_block(&self, f: &ast::Function) -> Result<ir::Block<'c>> {
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

    fn compile_value_expr<'a>(
        &self,
        block: &'a ir::Block<'c>,
        lvars: &mut TrainMap<String, ir::Value<'c, 'a>>,
        s_expr: &ast::SpannedExpr<'a>,
    ) -> Result<ir::Value<'c, 'a>> {
        match self.compile_expr(block, lvars, s_expr)? {
            Some(v) => Ok(v),
            None => Err(anyhow!("this expression does not have value")),
        }
    }

    fn compile_expr<'a>(
        &self,
        block: &'a ir::Block<'c>,
        lvars: &mut TrainMap<String, ir::Value<'c, 'a>>,
        s_expr: &ast::SpannedExpr<'a>,
    ) -> Result<Option<ir::Value<'c, 'a>>> {
        match s_expr {
            (ast::Expr::Number(n), pos) => self.compile_number(block, *n, pos),
            (ast::Expr::VarRef(name), pos) => self.compile_varref(block, lvars, name, pos),
            (ast::Expr::OpCall(op, lhs, rhs), pos) => {
                self.compile_op_call(block, lvars, op, lhs, rhs, pos)
            }
            (ast::Expr::FunCall(fexpr, arg_exprs), pos) => {
                self.compile_funcall(block, lvars, fexpr, arg_exprs, pos)
            }
            (ast::Expr::If(cond, then, els), pos) => {
                self.compile_if(block, lvars, cond, then, els, pos)
            }
            (ast::Expr::While(cond, exprs), pos) => {
                self.compile_while(block, lvars, cond, exprs, pos)
            }
            (ast::Expr::Alloc(name), pos) => self.compile_alloc(block, lvars, name, pos),
            (ast::Expr::Assign(name, rhs), pos) => {
                self.compile_assign(block, lvars, name, rhs, pos)
            }
            (ast::Expr::Return(val_expr), pos) => self.compile_return(block, lvars, val_expr, pos),
            //ast::Expr::Para(exprs) => self.compile_para(block, lvars, exprs),
            _ => todo!("{:?}", s_expr),
        }
    }

    fn compile_op_call<'a>(
        &self,
        block: &'a ir::Block<'c>,
        lvars: &mut TrainMap<String, ir::Value<'c, 'a>>,
        operator: &str,
        lhs: &ast::SpannedExpr<'a>,
        rhs: &ast::SpannedExpr<'a>,
        pos: &ast::Span,
    ) -> Result<Option<ir::Value<'c, 'a>>> {
        let f = match operator {
            "+" => dialect::arith::addi,
            "-" => dialect::arith::subi,
            "*" => dialect::arith::muli,
            "/" => dialect::arith::divsi,
            _ => return self.compile_cmp(block, lvars, operator, lhs, rhs, pos),
        };
        let op = f(
            self.compile_value_expr(block, lvars, lhs)?,
            self.compile_value_expr(block, lvars, rhs)?,
            self.loc(pos),
        );
        Ok(Some(val(block.append_operation(op))))
    }

    fn compile_cmp<'a>(
        &self,
        block: &'a ir::Block<'c>,
        lvars: &mut TrainMap<String, ir::Value<'c, 'a>>,
        operator: &str,
        lhs: &ast::SpannedExpr<'a>,
        rhs: &ast::SpannedExpr<'a>,
        pos: &ast::Span,
    ) -> Result<Option<ir::Value<'c, 'a>>> {
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
            self.compile_value_expr(block, lvars, lhs)?,
            self.compile_value_expr(block, lvars, rhs)?,
            self.loc(pos),
        );
        Ok(Some(val(block.append_operation(op))))
    }

    fn compile_funcall<'a>(
        &self,
        block: &'a ir::Block<'c>,
        lvars: &mut TrainMap<String, ir::Value<'c, 'a>>,
        fexpr: &ast::SpannedExpr<'a>,
        arg_exprs: &[ast::SpannedExpr<'a>],
        pos: &ast::Span,
    ) -> Result<Option<ir::Value<'c, 'a>>> {
        let fun_ty = match fexpr {
            (ast::Expr::VarRef(s), _) => {
                if let Some(t) = self.sigs.get(s) {
                    t
                } else {
                    todo!()
                }
            }
            _ => return Err(anyhow!("not a function?")),
        };

        let f = self.compile_value_expr(block, lvars, fexpr)?;

        let mut args = vec![];
        for e in arg_exprs {
            args.push(self.compile_value_expr(block, lvars, e)?.into());
        }

        let result_types = fun_ty
            .param_tys
            .iter()
            .map(|t| self.mlir_type(t))
            .collect::<Result<Vec<_>>>()?;
        let op = dialect::func::call_indirect(f, &args, &result_types, self.loc(pos));
        Ok(Some(val(block.append_operation(op))))
    }

    fn compile_if<'a>(
        &self,
        block: &'a ir::Block<'c>,
        lvars: &mut TrainMap<String, ir::Value<'c, 'a>>,
        cond_expr: &ast::SpannedExpr<'a>,
        then: &[ast::SpannedExpr<'a>],
        els: &Option<Vec<ast::SpannedExpr<'a>>>,
        pos: &ast::Span,
    ) -> Result<Option<ir::Value<'c, 'a>>> {
        let cond_result = self.compile_value_expr(block, lvars, cond_expr)?;
        let then_region = {
            let region = ir::Region::new();
            region.append_block(self.compile_exprs(lvars, then, true)?);
            region
        };
        let else_region = {
            let region = ir::Region::new();
            region.append_block(self.compile_exprs(
                lvars,
                if let Some(v) = els { v } else { &[] },
                true,
            )?);
            region
        };
        let op = dialect::scf::r#if(
            cond_result,
            Default::default(),
            then_region,
            else_region,
            self.loc(pos),
        );
        block.append_operation(op);
        Ok(None)
    }

    fn compile_while<'a>(
        &self,
        block: &'a ir::Block<'c>,
        lvars: &mut TrainMap<String, ir::Value<'c, 'a>>,
        cond_expr: &ast::SpannedExpr<'a>,
        exprs: &[ast::SpannedExpr<'a>],
        pos: &ast::Span,
    ) -> Result<Option<ir::Value<'c, 'a>>> {
        let before_region = {
            let region = ir::Region::new();
            let block = ir::Block::new(&[]);
            let mut lvars = lvars.fork();
            let v = self.compile_value_expr(&block, &mut lvars, cond_expr)?;
            block.append_operation(dialect::scf::condition(v, &[], self.loc(pos)));
            region.append_block(block);
            region
        };
        let after_region = {
            let region = ir::Region::new();
            let block = self.compile_exprs(lvars, exprs, true)?;
            region.append_block(block);
            region
        };
        block.append_operation(dialect::scf::r#while(
            &[],
            &[],
            before_region,
            after_region,
            self.loc(pos),
        ));
        Ok(None)
    }

    fn compile_alloc<'a>(
        &self,
        block: &'a ir::Block<'c>,
        lvars: &mut TrainMap<String, ir::Value<'c, 'a>>,
        name: &str,
        pos: &ast::Span,
    ) -> Result<Option<ir::Value<'c, 'a>>> {
        let op = dialect::memref::alloca(
            &self.context,
            MemRefType::new(self.int_type().into(), &[], None, None),
            &[],
            &[],
            None,
            self.loc(pos),
        );
        let v = val(block.append_operation(op));
        lvars.insert(name.to_string(), v.clone());
        Ok(Some(v))
    }

    fn compile_assign<'a>(
        &self,
        block: &'a ir::Block<'c>,
        lvars: &mut TrainMap<String, ir::Value<'c, 'a>>,
        name: &str,
        rhs: &ast::SpannedExpr<'a>,
        pos: &ast::Span,
    ) -> Result<Option<ir::Value<'c, 'a>>> {
        let rhs_result = self.compile_value_expr(block, lvars, rhs)?;
        let Some(lvar) = lvars.get(name) else {
            return Err(anyhow!("unknown lvar {name}"));
        };
        let op = dialect::memref::store(rhs_result, *lvar, &[], self.loc(pos));
        block.append_operation(op);
        Ok(None)
    }

    fn compile_return<'a>(
        &self,
        block: &'a ir::Block<'c>,
        lvars: &mut TrainMap<String, ir::Value<'c, 'a>>,
        expr: &ast::SpannedExpr<'a>,
        pos: &ast::Span,
    ) -> Result<Option<ir::Value<'c, 'a>>> {
        let v = self.compile_value_expr(block, lvars, expr)?;
        let op = dialect::func::r#return(&[v], self.loc(pos));
        block.append_operation(op);
        Ok(None)
    }

    fn compile_varref<'a>(
        &self,
        block: &'a ir::Block<'c>,
        lvars: &mut TrainMap<String, ir::Value<'c, 'a>>,
        name: &str,
        pos: &ast::Span,
    ) -> Result<Option<ir::Value<'c, 'a>>> {
        if let Some(fun_ty) = self.sigs.get(name) {
            let op = dialect::func::constant(
                &self.context,
                FlatSymbolRefAttribute::new(self.context, name),
                self.function_type(fun_ty)?,
                self.loc(pos),
            );
            Ok(Some(val(block.append_operation(op))))
        } else if let Some(v) = lvars.get(name) {
            let op = dialect::memref::load(v.clone(), &[], self.loc(pos));
            Ok(Some(val(block.append_operation(op))))
        } else {
            Err(anyhow!("unknown variable `{name}'"))
        }
    }

    fn compile_number<'a>(
        &self,
        block: &'a ir::Block<'c>,
        n: i64,
        pos: &ast::Span,
    ) -> Result<Option<ir::Value<'c, 'a>>> {
        Ok(Some(val(block.append_operation(self.const_int(n, pos)))))
    }

    /// Returns a newly created region that contains `exprs`.
    fn compile_exprs<'a>(
        &self,
        lvars: &mut TrainMap<String, ir::Value<'c, '_>>,
        exprs: &[ast::SpannedExpr<'a>],
        terminate: bool,
    ) -> Result<ir::Block<'c>> {
        let block = ir::Block::new(&[]);
        let mut lvars = lvars.fork();
        for expr in exprs {
            self.compile_expr(&block, &mut lvars, expr)?;
        }
        if terminate {
            let op = dialect::scf::r#yield(&[], self.unknown_location());
            block.append_operation(op);
        }
        Ok(block)
    }

    //            ast::Expr::Para(exprs) => {
    //                let block = ir::Block::new(&[]);
    //                for expr in exprs {
    //                    self.compile_expr(&block, lvars, expr)?;
    //                }
    //                let zero = self.compile_number(&block, 0);
    //                block.append_operation(
    //                    r#async::YieldOp::builder(&self.context, self.loc(pos))
    //                        .operands(&[val(&zero)])
    //                        .build()
    //                        .into(),
    //                );
    //                let region = ir::Region::new();
    //                region.append_block(block);
    //                r#async::ExecuteOp::builder(&self.context, self.loc(pos))
    //                    .token(Type::parse(&self.context, "!async.token").unwrap())
    //                    .body_results(&[Type::parse(&self.context, "!async.value<i64>").unwrap()])
    //                    .dependencies(&[])
    //                    .body_operands(&[])
    //                    .body_region(region)
    //                    .build()
    //                    .into()
    //            }

    fn const_int(&self, n: i64, pos: &ast::Span) -> ir::Operation<'c> {
        dialect::arith::constant(
            &self.context,
            IntegerAttribute::new(n, self.int_type().into()).into(),
            self.loc(pos),
        )
    }

    fn function_type(&self, fun_ty: &ast::FunTy) -> Result<ir::r#type::FunctionType<'c>> {
        let param_tys = self.mlir_types(&fun_ty.param_tys)?;
        let ret_ty = self.mlir_type(&fun_ty.ret_ty)?;
        Ok(FunctionType::new(&self.context, &param_tys, &[ret_ty]).into())
    }

    fn mlir_types(&self, tys: &[ast::Ty]) -> Result<Vec<ir::Type<'c>>> {
        tys.iter().map(|x| self.mlir_type(x)).collect()
    }

    fn mlir_type(&self, ty: &ast::Ty) -> Result<ir::Type<'c>> {
        let t = match ty {
            ast::Ty::Raw(s) => match &s[..] {
                "none" => Type::none(&self.context).into(),
                "int" => self.int_type().into(),
                _ => return Err(anyhow!("unknown type `{}'", s)),
            },
            //ast::Ty::Fun(fun_ty) => self.function_type(fun_ty)?.into(),
        };
        Ok(t)
    }

    fn int_type(&self) -> ir::Type<'c> {
        ir::r#type::IntegerType::new(&self.context, 64).into()
    }

    fn identifier(&self, s: &str) -> ir::Identifier<'c> {
        ir::Identifier::new(&self.context, s)
    }

    fn str_attr(&self, s: &str) -> StringAttribute<'c> {
        StringAttribute::new(&self.context, s)
    }

    fn loc(&self, span: &ast::Span) -> ir::Location<'c> {
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

    fn unknown_location(&self) -> ir::Location<'c> {
        ir::Location::unknown(&self.context)
    }
}
