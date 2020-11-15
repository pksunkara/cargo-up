use insta::assert_snapshot;
mod utils;

#[test]
fn test_init() {
    let (out, err) = utils::run_upgrader("init", "0.5.0", false);
    assert_snapshot!(out);
    assert_snapshot!(err);
}
