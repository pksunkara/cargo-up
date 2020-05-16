use crate::{
    ra_hir::Semantics,
    ra_ide_db::RootDatabase,
    ra_syntax::{
        ast::{self, AstNode},
        SyntaxKind, SyntaxNode,
    },
};

macro_rules! visit {
    ($method:ident, $node:ident) => {
        fn $method(&mut self, _: &ast::$node, _semantics: &Semantics<RootDatabase>) {}
    };
}

pub trait Visitor {
    fn visit(&mut self, node: &SyntaxNode, semantics: &Semantics<RootDatabase>) {
        match node.kind() {
            SyntaxKind::SOURCE_FILE => {
                self.visit_source_file(&ast::SourceFile::cast(node.clone()).unwrap(), &semantics)
            }
            SyntaxKind::METHOD_CALL_EXPR => self.visit_method_call_expr(
                &ast::MethodCallExpr::cast(node.clone()).unwrap(),
                &semantics,
            ),
            SyntaxKind::FIELD_EXPR => {
                self.visit_field_expr(&ast::FieldExpr::cast(node.clone()).unwrap(), &semantics)
            }
            _ => {}
        };

        for child in node.children() {
            self.visit(&child, &semantics);
        }
    }

    visit!(visit_source_file, SourceFile);
    visit!(visit_method_call_expr, MethodCallExpr);
    visit!(visit_field_expr, FieldExpr);
}
