use ra_ap_hir;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::{
    ast::{self, AstNode},
    SyntaxKind, SyntaxNode,
};

pub(crate) const INTERNAL_ERR: &'static str =
    "Internal error message. Please create an issue on https://github.com/pksunkara/cargo-up";

pub use ra_ap_syntax;

pub type Semantics<'db> = ra_ap_hir::Semantics<'db, RootDatabase>;

macro_rules! visitor {
    () => {};
    ($($kind:ident => $method:ident as $node:ident,)*) => {
        pub trait Visitor {
            fn visit(&mut self, node: &SyntaxNode, semantics: &Semantics) {
                match node.kind() {
                    $(SyntaxKind::$kind => self.$method(
                        &ast::$node::cast(node.clone()).expect(INTERNAL_ERR),
                        &semantics,
                    ),)*
                    _ => {},
                }

                for child in node.children() {
                    self.visit(&child, semantics);
                }

                self.post_visit(node, semantics);
            }

            fn post_visit(&mut self, _node: &SyntaxNode, _semantics: &Semantics) {}

            $(fn $method(&mut self, _ast: &ast::$node, _semantics: &Semantics) {})*
        }
    };
}

visitor!(
    SOURCE_FILE => visit_source_file as SourceFile,
    STRUCT => visit_struct as Struct,
    UNION => visit_union as Union,
    ENUM => visit_enum as Enum,
    FN => visit_fn as Fn,
    RET_TYPE => visit_ret_type as RetType,
    EXTERN_CRATE => visit_extern_crate as ExternCrate,
    MODULE => visit_module as Module,
    USE => visit_use as Use,
    STATIC => visit_static as Static,
    CONST => visit_const as Const,
    TRAIT => visit_trait as Trait,
    IMPL => visit_impl as Impl,
    TYPE_ALIAS => visit_type_alias as TypeAlias,
    MACRO_CALL => visit_macro_call as MacroCall,
    TOKEN_TREE => visit_token_tree as TokenTree,
    PAREN_TYPE => visit_paren_type as ParenType,
    TUPLE_TYPE => visit_tuple_type as TupleType,
    NEVER_TYPE => visit_never_type as NeverType,
    PATH_TYPE => visit_path_type as PathType,
    PTR_TYPE => visit_ptr_type as PtrType,
    ARRAY_TYPE => visit_array_type as ArrayType,
    SLICE_TYPE => visit_slice_type as SliceType,
    REF_TYPE => visit_ref_type as RefType,
    INFER_TYPE => visit_infer_type as InferType,
    FN_PTR_TYPE => visit_fn_ptr_type as FnPtrType,
    FOR_TYPE => visit_for_type as ForType,
    IMPL_TRAIT_TYPE => visit_impl_trait_type as ImplTraitType,
    DYN_TRAIT_TYPE => visit_dyn_trait_type as DynTraitType,
    OR_PAT => visit_or_pat as OrPat,
    PAREN_PAT => visit_paren_pat as ParenPat,
    REF_PAT => visit_ref_pat as RefPat,
    BOX_PAT => visit_box_pat as BoxPat,
    IDENT_PAT => visit_ident_pat as IdentPat,
    WILDCARD_PAT => visit_wildcard_pat as WildcardPat,
    REST_PAT => visit_rest_pat as RestPat,
    PATH_PAT => visit_path_pat as PathPat,
    RECORD_PAT => visit_record_pat as RecordPat,
    RECORD_PAT_FIELD_LIST => visit_record_pat_field_list as RecordPatFieldList,
    RECORD_PAT_FIELD => visit_record_pat_field as RecordPatField,
    TUPLE_STRUCT_PAT => visit_tuple_struct_pat as TupleStructPat,
    TUPLE_PAT => visit_tuple_pat as TuplePat,
    SLICE_PAT => visit_slice_pat as SlicePat,
    RANGE_PAT => visit_range_pat as RangePat,
    LITERAL_PAT => visit_literal_pat as LiteralPat,
    MACRO_PAT => visit_macro_pat as MacroPat,
    TUPLE_EXPR => visit_tuple_expr as TupleExpr,
    ARRAY_EXPR => visit_array_expr as ArrayExpr,
    PAREN_EXPR => visit_paren_expr as ParenExpr,
    PATH_EXPR => visit_path_expr as PathExpr,
    CLOSURE_EXPR => visit_closure_expr as ClosureExpr,
    IF_EXPR => visit_if_expr as IfExpr,
    WHILE_EXPR => visit_while_expr as WhileExpr,
    CONDITION => visit_condition as Condition,
    LOOP_EXPR => visit_loop_expr as LoopExpr,
    FOR_EXPR => visit_for_expr as ForExpr,
    CONTINUE_EXPR => visit_continue_expr as ContinueExpr,
    BREAK_EXPR => visit_break_expr as BreakExpr,
    LABEL => visit_label as Label,
    BLOCK_EXPR => visit_block_expr as BlockExpr,
    RETURN_EXPR => visit_return_expr as ReturnExpr,
    MATCH_EXPR => visit_match_expr as MatchExpr,
    MATCH_ARM_LIST => visit_match_arm_list as MatchArmList,
    MATCH_ARM => visit_match_arm as MatchArm,
    MATCH_GUARD => visit_match_guard as MatchGuard,
    RECORD_EXPR => visit_record_expr as RecordExpr,
    RECORD_EXPR_FIELD_LIST => visit_record_expr_field_list as RecordExprFieldList,
    RECORD_EXPR_FIELD => visit_record_expr_field as RecordExprField,
    EFFECT_EXPR => visit_effect_expr as EffectExpr,
    BOX_EXPR => visit_box_expr as BoxExpr,
    CALL_EXPR => visit_call_expr as CallExpr,
    INDEX_EXPR => visit_index_expr as IndexExpr,
    METHOD_CALL_EXPR => visit_method_call_expr as MethodCallExpr,
    FIELD_EXPR => visit_field_expr as FieldExpr,
    AWAIT_EXPR => visit_await_expr as AwaitExpr,
    TRY_EXPR => visit_try_expr as TryExpr,
    CAST_EXPR => visit_cast_expr as CastExpr,
    REF_EXPR => visit_ref_expr as RefExpr,
    PREFIX_EXPR => visit_prefix_expr as PrefixExpr,
    RANGE_EXPR => visit_range_expr as RangeExpr,
    BIN_EXPR => visit_bin_expr as BinExpr,
    EXTERN_BLOCK => visit_extern_block as ExternBlock,
    EXTERN_ITEM_LIST => visit_extern_item_list as ExternItemList,
    VARIANT => visit_variant as Variant,
    RECORD_FIELD_LIST => visit_record_field_list as RecordFieldList,
    RECORD_FIELD => visit_record_field as RecordField,
    TUPLE_FIELD_LIST => visit_tuple_field_list as TupleFieldList,
    TUPLE_FIELD => visit_tuple_field as TupleField,
    VARIANT_LIST => visit_variant_list as VariantList,
    ITEM_LIST => visit_item_list as ItemList,
    ASSOC_ITEM_LIST => visit_assoc_item_list as AssocItemList,
    ATTR => visit_attr as Attr,
    USE_TREE => visit_use_tree as UseTree,
    USE_TREE_LIST => visit_use_tree_list as UseTreeList,
    PATH => visit_path as Path,
    PATH_SEGMENT => visit_path_segment as PathSegment,
    LITERAL => visit_literal as Literal,
    RENAME => visit_rename as Rename,
    VISIBILITY => visit_visibility as Visibility,
    WHERE_CLAUSE => visit_where_clause as WhereClause,
    WHERE_PRED => visit_where_pred as WherePred,
    ABI => visit_abi as Abi,
    NAME => visit_name as Name,
    NAME_REF => visit_name_ref as NameRef,
    LET_STMT => visit_let_stmt as LetStmt,
    EXPR_STMT => visit_expr_stmt as ExprStmt,
    GENERIC_PARAM_LIST => visit_generic_param_list as GenericParamList,
    GENERIC_PARAM => visit_generic_param as GenericParam,
    LIFETIME_PARAM => visit_lifetime_param as LifetimeParam,
    TYPE_PARAM => visit_type_param as TypeParam,
    CONST_PARAM => visit_const_param as ConstParam,
    GENERIC_ARG_LIST => visit_generic_arg_list as GenericArgList,
    LIFETIME_ARG => visit_lifetime_arg as LifetimeArg,
    TYPE_ARG => visit_type_arg as TypeArg,
    ASSOC_TYPE_ARG => visit_assoc_type_arg as AssocTypeArg,
    CONST_ARG => visit_const_arg as ConstArg,
    PARAM_LIST => visit_param_list as ParamList,
    PARAM => visit_param as Param,
    SELF_PARAM => visit_self_param as SelfParam,
    ARG_LIST => visit_arg_list as ArgList,
    TYPE_BOUND => visit_type_bound as TypeBound,
    TYPE_BOUND_LIST => visit_type_bound_list as TypeBoundList,
    MACRO_ITEMS => visit_macro_items as MacroItems,
    MACRO_STMTS => visit_macro_stmts as MacroStmts,
);
