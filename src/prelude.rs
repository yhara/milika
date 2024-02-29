use crate::hir;

/// Returns the functions needed to run the Milika program.
pub fn prelude_funcs(main_is_async: bool) -> Vec<hir::Function> {
    vec![hir_chiika_start_user(main_is_async)]
}

fn hir_chiika_start_user(is_async: bool) -> hir::Function {
    hir::Function {
        name: "chiika_start_user".to_string(),
        params: vec![
            hir::Param {
                name: "$env".to_string(),
                ty: hir::Ty::ChiikaEnv,
            },
            hir::Param {
                name: "$cont".to_string(),
                ty: hir::Ty::ChiikaCont,
            },
        ],
        ret_ty: hir::Ty::RustFuture,
        body_stmts: {
            let stmt = if is_async {
                // Generate `return $cont($env, chiika_main());`
                let chiika_main = hir::Expr::func_ref(
                    "chiika_main",
                    hir::FunTy::sync(
                        vec![hir::Ty::ChiikaEnv, hir::Ty::ChiikaCont],
                        hir::Ty::RustFuture,
                    ),
                );
                hir::Expr::return_(hir::Expr::fun_call(
                    hir::Expr::arg_ref(1, hir::Ty::ChiikaCont),
                    vec![
                        hir::Expr::arg_ref(0, hir::Ty::ChiikaEnv),
                        hir::Expr::fun_call(chiika_main, vec![]),
                    ],
                ))
            } else {
                // Generate `return chiika_main($env, $cont);`
                let chiika_main = hir::Expr::func_ref(
                    "chiika_main",
                    hir::FunTy::sync(
                        vec![hir::Ty::ChiikaEnv, hir::Ty::ChiikaCont],
                        hir::Ty::Int, //TODO: change to Void
                    ),
                );
                hir::Expr::return_(hir::Expr::fun_call(
                    chiika_main,
                    vec![
                        hir::Expr::arg_ref(0, hir::Ty::ChiikaEnv),
                        hir::Expr::arg_ref(1, hir::Ty::ChiikaCont),
                    ],
                ))
            };
            vec![stmt]
        },
    }
}
