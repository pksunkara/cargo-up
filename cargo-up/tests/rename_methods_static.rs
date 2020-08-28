use insta::assert_snapshot;
mod utils;

#[test]
fn test_rename_methods_static() {
    let (out, err) = utils::run_upgrader("rename_methods_static", "0.3.0", true);
    assert_snapshot!(out);
    assert_snapshot!(err);
}
