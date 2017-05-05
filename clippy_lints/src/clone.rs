use rustc::hir::*;
use rustc::lint::*;

/// **What it does:** This lint suggests using clone_from()
///
/// **Why is this bad?** This makes code harder to read
///
/// **Example:**
/// ```rust
/// a = b.clone()
/// // should be
/// a.clone_from(&b)
/// ```
declare_lint! {
    pub CLONE_FROM,
    Warn,
    "Warn on uses of a = b.clone(), suggesting to use a = clone_from(&b)"
}

#[derive(Copy,Clone)]
pub struct From;

impl LintPass for From {
    fn get_lints(&self) -> LintArray {
        lint_array!(CLONE_FROM)
    }
}

impl<'a, 'tcx> LateLintPass<'a, 'tcx> for From {
    fn check_expr(&mut self, cx: &LateContext<'a, 'tcx>, expr: &'tcx Expr) {
        // insert check here.
    }

    fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx Item) {
        // need this for debugging?
    }
}
