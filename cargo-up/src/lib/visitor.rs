use crate::{
    ra_hir::Semantics,
    ra_ide_db::RootDatabase,
    ra_syntax::{
        ast::{self, AstNode},
        SyntaxKind, SyntaxNode,
    },
};

pub trait Visitor {
    fn visit(&mut self, node: &SyntaxNode, semantics: &Semantics<RootDatabase>) {
        match node.kind() {
            SyntaxKind::SOURCE_FILE => {
                self.visit_source_file(ast::SourceFile::cast(node.clone()).unwrap(), &semantics)
            }
            SyntaxKind::METHOD_CALL_EXPR => self.visit_method_call_expr(
                ast::MethodCallExpr::cast(node.clone()).unwrap(),
                &semantics,
            ),
            _ => {}
        };

        for child in node.children() {
            self.visit(&child, &semantics);
        }
    }

    fn visit_source_file(&mut self, _: ast::SourceFile, _: &Semantics<RootDatabase>) {}
    fn visit_method_call_expr(&mut self, _: ast::MethodCallExpr, _: &Semantics<RootDatabase>) {}
}
