use insta::assert_snapshot;
mod utils;

#[test]
fn test_no_version() {
    let (out, err) = utils::run_upgrader("rename_member", "0.2.1");
    assert_snapshot!(out);
    assert_snapshot!(err);
}