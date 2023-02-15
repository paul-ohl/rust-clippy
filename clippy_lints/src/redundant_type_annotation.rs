#![allow(unused_imports, unused_variables)]

use rustc_ast::{ExprPrecedence, LitKind};
use rustc_hir::{Block, Expr, ExprKind, HirId, Local, Node, PatKind, PathSegment, StmtKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint_pass, declare_tool_lint};

use rustc_errors::Applicability;

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
    fn check_block(&mut self, ctx: &LateContext<'tcx>, block: &Block<'tcx>) {
        for stmt in block.stmts.iter() {
            if_chain! {
                if !stmt.span.from_expansion();
                if let StmtKind::Local(local) = stmt.kind;
                // The ty part
                if let Some(ty) = local.ty;
                if let rustc_hir::TyKind::Path(ty_path) = &ty.kind;
                if let rustc_hir::QPath::Resolved(_, ty_resolved_qpath) = ty_path;
                // The init part
                if let Some(init) = local.init;
                if let rustc_hir::ExprKind::Call(init_call, _) = init.kind;
                if let rustc_hir::ExprKind::Path(init_path) = &init_call.kind;
                if let rustc_hir::QPath::TypeRelative(init_ty, _) = init_path;
                if let rustc_hir::TyKind::Path(init_ty_path) = &init_ty.kind;
                if let rustc_hir::QPath::Resolved(_, init_ty_resolved_qpath) = init_ty_path;
                then {
                    // println!("Init:\n{:#?}\n===================\n", init_ty_resolved_qpath.res);
                    // This one is gooooood!!!!!!
                    // println!("Ty:\n{:#?}\n===================\n", ty_resolved_qpath.res);
                    println!("Result: {}", ty_resolved_qpath.res == init_ty_resolved_qpath.res);
                }
                else {
                    println!("False");
                }
            }
        }
    }
}
