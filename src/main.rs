mod ast;
mod asyncness_check;
mod compiler;
mod parser;
use anyhow::{bail, Context, Result};
use ariadne::{Label, Report, ReportKind, Source};
use chumsky::Parser;
use parser::parser;

fn render_parse_error(src: &str, span: std::ops::Range<usize>, msg: String) -> String {
    let mut rendered = vec![];
    Report::build(ReportKind::Error, "", span.start)
        .with_message(msg.clone())
        .with_label(Label::new(("", span)).with_message(msg))
        .finish()
        .write(("", Source::from(src)), &mut rendered)
        .unwrap();
    String::from_utf8_lossy(&rendered).to_string()
}

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let Some(path) = args.get(1) else {
        bail!("usage: milika a.milika > a.mlir");
    };
    let src = std::fs::read_to_string(path).context(format!("failed to read {}", path))?;
    let ast = match parser().parse(src) {
        Ok(x) => x,
        Err(errs) => {
            let src = std::fs::read_to_string(path)?;
            let mut s = String::new();
            errs.into_iter().for_each(|e| {
                s += &render_parse_error(&src, e.span(), e.to_string());
            });
            bail!(s);
        }
    };
    compiler::run(ast)?;
    Ok(())
}

//use melior::{
//    dialect::{self, arith, func, DialectRegistry},
//    ir::{
//        attribute::{StringAttribute, TypeAttribute},
//        r#type::FunctionType,
//        *,
//    },
//    pass::{self, PassManager},
//    utility::{register_all_dialects, register_all_llvm_translations},
//};
//fn main() {
//    let registry = DialectRegistry::new();
//    register_all_dialects(&registry);
//
//    let context = melior::Context::new();
//    context.append_dialect_registry(&registry);
//    context.load_all_available_dialects();
//    register_all_llvm_translations(&context);
//
//    let location = Location::unknown(&context);
//    let mut module = Module::new(location);
//
//    let index_type = Type::index(&context);
//
//    module.body().append_operation(func::func(
//        &context,
//        StringAttribute::new(&context, "add"),
//        TypeAttribute::new(
//            FunctionType::new(&context, &[index_type, index_type], &[index_type]).into(),
//        ),
//        {
//            let block = Block::new(&[(index_type, location), (index_type, location)]);
//
//            let sum = block.append_operation(arith::addi(
//                block.argument(0).unwrap().into(),
//                block.argument(1).unwrap().into(),
//                location,
//            ));
//
//            block.append_operation(func::r#return(&[sum.result(0).unwrap().into()], location));
//
//            let region = Region::new();
//            region.append_block(block);
//            region
//        },
//        &[],
//        location,
//    ));
//
//    assert!(module.as_operation().verify());
//    module.as_operation().dump();
//    println!("--");
//
//    let pass_manager = PassManager::new(&context);
//    pass_manager.add_pass(pass::conversion::create_func_to_llvm());
//    pass_manager
//        .nested_under("func.func")
//        .add_pass(pass::conversion::create_arith_to_llvm());
//    pass_manager
//        .nested_under("func.func")
//        .add_pass(pass::conversion::create_index_to_llvm());
//    pass_manager.add_pass(pass::conversion::create_scf_to_control_flow());
//    pass_manager.add_pass(pass::conversion::create_control_flow_to_llvm());
//    pass_manager.add_pass(pass::conversion::create_finalize_mem_ref_to_llvm());
//    pass_manager.run(&mut module).unwrap();
//
//    assert!(module.as_operation().verify());
//    module.as_operation().dump();
//}
