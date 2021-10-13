use crate::{
    ra_ap_syntax::ast::{self, HasName},
    runner::{
        context::Context,
        helpers::{
            get_name, get_name_from_name, get_name_from_name_ref, get_name_from_path, run_hooks,
        },
    },
    utils::INTERNAL_ERR,
};

use ra_ap_hir::{Adt, AssocItem, ModuleDef, PathResolution};
use rust_visitor::{Options, Visitor};

impl<'a> Visitor for Context<'a> {
    fn visit_source_file(&mut self, _: &ast::SourceFile, _: &mut Options) {}

    fn visit_method_call_expr(&mut self, method_call_expr: &ast::MethodCallExpr, _: &mut Options) {
        let version = self.runner.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_method_call_expr {
            hook(&mut self.upgrader, method_call_expr, &self.semantics);
        }

        run_hooks(
            &mut self.upgrader,
            &self.semantics,
            &self.preloader.methods,
            &version.hook_method_call_expr_on,
            method_call_expr,
            |n| get_name_from_name_ref(n.name_ref()),
            |s, n| s.resolve_method_call(n),
        );
    }

    fn visit_call_expr(&mut self, call_expr: &ast::CallExpr, _: &mut Options) {
        let version = self.runner.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_call_expr {
            hook(&mut self.upgrader, call_expr, &self.semantics);
        }
    }

    fn visit_ident_pat(&mut self, ident_pat: &ast::IdentPat, _: &mut Options) {
        let version = self.runner.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_ident_pat {
            hook(&mut self.upgrader, ident_pat, &self.semantics);
        }

        run_hooks(
            &mut self.upgrader,
            &self.semantics,
            &self.preloader.variants,
            &version.hook_ident_pat_on,
            ident_pat,
            |n| get_name_from_name(n.name()),
            |s, n| {
                if let Some(ModuleDef::Variant(x)) = s.resolve_bind_pat_to_const(n) {
                    Some(x)
                } else {
                    None
                }
            },
        );
    }

    fn visit_path(&mut self, path: &ast::Path, _: &mut Options) {
        let version = self.runner.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_path {
            hook(&mut self.upgrader, path, &self.semantics);
        }

        run_hooks(
            &mut self.upgrader,
            &self.semantics,
            &self.preloader.structs,
            &version.hook_path_on,
            path,
            |n| get_name_from_path(Some(n.clone())),
            |s, n| {
                if let Some(PathResolution::Def(ModuleDef::Adt(Adt::Struct(x)))) = s.resolve_path(n)
                {
                    Some(x)
                } else {
                    None
                }
            },
        );
    }

    fn visit_path_expr(&mut self, path_expr: &ast::PathExpr, _: &mut Options) {
        let version = self.runner.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_path_expr {
            hook(&mut self.upgrader, path_expr, &self.semantics);
        }

        run_hooks(
            &mut self.upgrader,
            &self.semantics,
            &self.preloader.methods,
            &version.hook_path_expr_on,
            path_expr,
            |n| get_name_from_path(n.path()),
            |s, n| {
                if let Some(PathResolution::AssocItem(AssocItem::Function(x))) =
                    s.resolve_path(&n.path().expect(INTERNAL_ERR))
                {
                    Some(x)
                } else {
                    None
                }
            },
        );

        run_hooks(
            &mut self.upgrader,
            &self.semantics,
            &self.preloader.variants,
            &version.hook_path_expr_on,
            path_expr,
            |n| get_name_from_path(n.path()),
            |s, n| {
                if let Some(PathResolution::Def(ModuleDef::Variant(x))) =
                    s.resolve_path(&n.path().expect(INTERNAL_ERR))
                {
                    Some(x)
                } else {
                    None
                }
            },
        );
    }

    fn visit_path_pat(&mut self, path_pat: &ast::PathPat, _: &mut Options) {
        let version = self.runner.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_path_pat {
            hook(&mut self.upgrader, path_pat, &self.semantics);
        }

        run_hooks(
            &mut self.upgrader,
            &self.semantics,
            &self.preloader.variants,
            &version.hook_path_pat_on,
            path_pat,
            |n| get_name_from_path(n.path()),
            |s, n| {
                if let Some(PathResolution::Def(ModuleDef::Variant(x))) =
                    s.resolve_path(&n.path().expect(INTERNAL_ERR))
                {
                    Some(x)
                } else {
                    None
                }
            },
        );
    }

    fn visit_field_expr(&mut self, field_expr: &ast::FieldExpr, _: &mut Options) {
        let version = self.runner.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_field_expr {
            hook(&mut self.upgrader, field_expr, &self.semantics);
        }

        run_hooks(
            &mut self.upgrader,
            &self.semantics,
            &self.preloader.members,
            &version.hook_field_expr_on,
            field_expr,
            |n| get_name_from_name_ref(n.name_ref()),
            |s, n| s.resolve_field(n),
        );
    }

    fn visit_record_pat(&mut self, record_pat: &ast::RecordPat, _: &mut Options) {
        let version = self.runner.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_record_pat {
            hook(&mut self.upgrader, record_pat, &self.semantics);
        }

        run_hooks(
            &mut self.upgrader,
            &self.semantics,
            &self.preloader.variants,
            &version.hook_record_pat_on,
            record_pat,
            |n| get_name_from_path(n.path()),
            |s, n| {
                if let Some(PathResolution::Def(ModuleDef::Variant(x))) =
                    s.resolve_path(&n.path().expect(INTERNAL_ERR))
                {
                    Some(x)
                } else {
                    None
                }
            },
        );
    }

    fn visit_record_expr(&mut self, record_expr: &ast::RecordExpr, _: &mut Options) {
        let version = self.runner.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_record_expr {
            hook(&mut self.upgrader, record_expr, &self.semantics);
        }

        run_hooks(
            &mut self.upgrader,
            &self.semantics,
            &self.preloader.variants,
            &version.hook_record_expr_on,
            record_expr,
            |n| get_name_from_path(n.path()),
            |s, n| {
                if let Some(PathResolution::Def(ModuleDef::Variant(x))) =
                    s.resolve_path(&n.path().expect(INTERNAL_ERR))
                {
                    Some(x)
                } else {
                    None
                }
            },
        );
    }

    fn visit_record_expr_field(
        &mut self,
        record_expr_field: &ast::RecordExprField,
        _: &mut Options,
    ) {
        let version = self.runner.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_record_expr_field {
            hook(&mut self.upgrader, record_expr_field, &self.semantics);
        }

        run_hooks(
            &mut self.upgrader,
            &self.semantics,
            &self.preloader.members,
            &version.hook_record_expr_field_on,
            record_expr_field,
            |n| get_name_from_name_ref(n.field_name()),
            |s, n| {
                if let Some((x, _, _)) = s.resolve_record_field(n) {
                    Some(x)
                } else {
                    None
                }
            },
        );
    }

    fn visit_record_pat_field(&mut self, record_pat_field: &ast::RecordPatField, _: &mut Options) {
        let version = self.runner.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_record_pat_field {
            hook(&mut self.upgrader, record_pat_field, &self.semantics);
        }

        run_hooks(
            &mut self.upgrader,
            &self.semantics,
            &self.preloader.members,
            &version.hook_record_pat_field_on,
            record_pat_field,
            |n| get_name(n.field_name()),
            |s, n| s.resolve_record_pat_field(n),
        );
    }

    fn visit_tuple_struct_pat(&mut self, tuple_struct_pat: &ast::TupleStructPat, _: &mut Options) {
        let version = self.runner.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_tuple_struct_pat {
            hook(&mut self.upgrader, tuple_struct_pat, &self.semantics);
        }

        run_hooks(
            &mut self.upgrader,
            &self.semantics,
            &self.preloader.variants,
            &version.hook_tuple_struct_pat_on,
            tuple_struct_pat,
            |n| get_name_from_path(n.path()),
            |s, n| {
                if let Some(PathResolution::Def(ModuleDef::Variant(x))) =
                    s.resolve_path(&n.path().expect(INTERNAL_ERR))
                {
                    Some(x)
                } else {
                    None
                }
            },
        );
    }
}
