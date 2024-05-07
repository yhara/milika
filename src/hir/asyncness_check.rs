use crate::hir;
use crate::hir::visitor::HirVisitor;
use anyhow::Result;
use std::collections::HashMap;

/// Set Function.is_async to true or false.
/// This judgement is conservative because it is (possible, but) hard to
/// tell if a function is async or not when we support first-class functions.
/// It is safe to be false-positive (performance penalty aside).
pub fn run(hir: &mut hir::Program) {
    let mut known = HashMap::new();
    for e in &hir.externs {
        known.insert(e.name.clone(), e.is_async);
    }

    for f in hir.funcs.iter_mut() {
        // TODO: Better detection
        f.is_async = Some(Check::run(&f, &known).unwrap());
    }
}

pub struct Check<'a> {
    result: bool,
    known: &'a HashMap<String, bool>,
}
impl<'a> Check<'a> {
    pub fn run(f: &hir::Function, known: &HashMap<String, bool>) -> Result<bool> {
        let mut c = Check {
            result: false,
            known,
        };
        c.walk_exprs(&f.body_stmts)?;
        Ok(c.result)
    }

    fn check_fexpr(&mut self, fexpr: &hir::TypedExpr) {
        match fexpr {
            (hir::Expr::FuncRef(ref name), _) => {
                if let Some(false) = self.known.get(name) {
                    // Calling a non-async function
                } else {
                    // Conservatively assume it is async
                    self.result = true;
                }
            }
            _ => {
                // Conservatively assume it is async
                self.result = true;
            }
        }
    }
}
impl<'a> HirVisitor for Check<'a> {
    fn visit_expr(&mut self, texpr: &hir::TypedExpr) -> Result<()> {
        match texpr {
            (hir::Expr::FunCall(fexpr, _), _) => {
                self.check_fexpr(fexpr);
            }
            (hir::Expr::CondReturn(_, fexpr_t, _, fexpr_f, _), _) => {
                self.check_fexpr(fexpr_t);
                self.check_fexpr(fexpr_f);
            }
            _ => {}
        }
        Ok(())
    }
}
