use crate::{
    ra_hir::Semantics,
    ra_ide_db::RootDatabase,
    ra_syntax::{
        ast::{self, AstNode},
        SyntaxKind, SyntaxNode,
    },
    utils::INTERNAL_ERR,
};

macro_rules! visit {
    ($method:ident, $node:ident) => {
        fn $method(&mut self, _: &ast::$node, _semantics: &Semantics<RootDatabase>) {}
    };
}

pub trait Visitor {
    fn visit(&mut self, node: &SyntaxNode, semantics: &Semantics<RootDatabase>) {
        match node.kind() {
            SyntaxKind::SOURCE_FILE => self.visit_source_file(
                &ast::SourceFile::cast(node.clone()).expect(INTERNAL_ERR),
                &semantics,
            ),
            SyntaxKind::METHOD_CALL_EXPR => self.visit_method_call_expr(
                &ast::MethodCallExpr::cast(node.clone()).expect(INTERNAL_ERR),
                &semantics,
            ),
            SyntaxKind::CALL_EXPR => self.visit_call_expr(
                &ast::CallExpr::cast(node.clone()).expect(INTERNAL_ERR),
                &semantics,
            ),
            SyntaxKind::PATH_EXPR => self.visit_path_expr(
                &ast::PathExpr::cast(node.clone()).expect(INTERNAL_ERR),
                &semantics,
            ),
            SyntaxKind::FIELD_EXPR => self.visit_field_expr(
                &ast::FieldExpr::cast(node.clone()).expect(INTERNAL_ERR),
                &semantics,
            ),
            SyntaxKind::RECORD_LIT => self.visit_record_lit(
                &ast::RecordLit::cast(node.clone()).expect(INTERNAL_ERR),
                &semantics,
            ),
            SyntaxKind::RECORD_FIELD => self.visit_record_field(
                &ast::RecordField::cast(node.clone()).expect(INTERNAL_ERR),
                &semantics,
            ),
            SyntaxKind::RECORD_FIELD_PAT => self.visit_record_field_pat(
                &ast::RecordFieldPat::cast(node.clone()).expect(INTERNAL_ERR),
                &semantics,
            ),
            _ => {}
        };

        for child in node.children() {
            self.visit(&child, &semantics);
        }
    }

    visit!(visit_source_file, SourceFile);
    visit!(visit_method_call_expr, MethodCallExpr);
    visit!(visit_call_expr, CallExpr);
    visit!(visit_path_expr, PathExpr);
    visit!(visit_field_expr, FieldExpr);
    visit!(visit_record_lit, RecordLit);
    visit!(visit_record_field, RecordField);
    visit!(visit_record_field_pat, RecordFieldPat);
}
