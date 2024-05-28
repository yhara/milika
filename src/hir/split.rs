use crate::hir;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Program {
    pub externs: Vec<hir::Extern>,
    // Grouped by the original function. Group size is one if the original
    // is a sync function.
    pub funcs: Vec<Vec<hir::Function>>,
    _marker: std::marker::PhantomData<*const ()>,
}

impl From<hir::Program> for Program {
    fn from(hir_program: hir::Program) -> Self {
        Self::new(
            hir_program.externs,
            hir_program
                .funcs
                .into_iter()
                .map(|func| vec![func])
                .collect(),
        )
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for e in &self.externs {
            write!(f, "{}", e)?;
        }
        for group in &self.funcs {
            for func in group {
                write!(f, "{}", func)?;
            }
        }
        write!(f, "")
    }
}

impl Program {
    pub fn new(externs: Vec<hir::Extern>, funcs: Vec<Vec<hir::Function>>) -> Self {
        debug_assert!(has_uniq_names(&funcs));
        Program {
            externs,
            funcs,
            _marker: std::marker::PhantomData,
        }
    }
}

fn has_uniq_names(funcs: &[Vec<hir::Function>]) -> bool {
    let mut names = std::collections::HashSet::new();
    for group in funcs {
        for func in group {
            if !names.insert(func.name.clone()) {
                return false;
            }
        }
    }
    true
}
