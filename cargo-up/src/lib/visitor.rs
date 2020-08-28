use crate::{
    ra_ap_syntax::{
        ast::{self, AstNode},
        SyntaxKind, SyntaxNode,
    },
    utils::INTERNAL_ERR,
};
use ra_ap_hir;
use ra_ap_ide_db::RootDatabase;

pub type Semantics<'db> = ra_ap_hir::Semantics<'db, RootDatabase>;

macro_rules! visit {
    ($method:ident, $node:ident) => {
        fn $method(&mut self, _: &ast::$node, _semantics: &Semantics) {}
    };
}

pub trait Visitor {
    fn visit(&mut self, node: &SyntaxNode, semantics: &Semantics) {
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
            SyntaxKind::RECORD_PAT => self.visit_record_pat(
                &ast::RecordPat::cast(node.clone()).expect(INTERNAL_ERR),
                &semantics,
            ),
            SyntaxKind::RECORD_EXPR => self.visit_record_expr(
                &ast::RecordExpr::cast(node.clone()).expect(INTERNAL_ERR),
                &semantics,
            ),
            SyntaxKind::RECORD_EXPR_FIELD => self.visit_record_expr_field(
                &ast::RecordExprField::cast(node.clone()).expect(INTERNAL_ERR),
                &semantics,
            ),
            SyntaxKind::RECORD_PAT_FIELD => self.visit_record_pat_field(
                &ast::RecordPatField::cast(node.clone()).expect(INTERNAL_ERR),
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
    visit!(visit_record_pat, RecordPat);
    visit!(visit_record_expr, RecordExpr);
    visit!(visit_record_expr_field, RecordExprField);
    visit!(visit_record_pat_field, RecordPatField);
}
