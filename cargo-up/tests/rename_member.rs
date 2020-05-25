use insta::assert_snapshot;
mod utils;

#[test]
fn test_rename_member() {
    let (out, err) = utils::run_upgrader("rename_member", "0.3.0");
    assert_snapshot!(out);
    assert_snapshot!(err);
}