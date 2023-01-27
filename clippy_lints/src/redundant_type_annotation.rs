use clippy_utils::diagnostics::span_lint_and_help;
use rustc_ast::ast::*;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use regex_syntax::hir;

declare_clippy_lint! {
    /// ### What it does
    /// Warns about needless type annotations.
    ///
    /// ### Why is this bad?
    /// The code is harder to modify, longer and less idiomatic
    ///
    /// ### Example
    /// ```rust,ignore
    /// let foo: String = String::new();
    /// ```
    ///
    /// Use instead:
    /// ```rust,ignore
    /// let foo = String::new();
    /// ```
    #[clippy::version = "1.68.0"]
    pub REDUNDANT_TYPE_ANNOTATION,
    style,
    "default lint description"
}
declare_lint_pass!(RedundantTypeAnnotation => [REDUNDANT_TYPE_ANNOTATION]);

impl<'tcx> LateLintPass<'tcx> for RedundantTypeAnnotation  {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx hir::Expr<'_>) {
        let ty = cx.typeck_results().expr_ty(expr);
        span_lint_and_help(
            cx,
            REDUNDANT_TYPE_ANNOTATION,
            expr.span,
            "type is reduntantly annotated",
            None,
            "consider removing the type annotation",
            );
    }
}
