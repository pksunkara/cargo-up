use insta::assert_snapshot;
mod utils;

#[test]
fn test_minimum_version() {
    let (out, err) = utils::run_upgrader("minimum_version", "0.3.0", false);
    assert_snapshot!(out);
    assert_snapshot!(err);
}
