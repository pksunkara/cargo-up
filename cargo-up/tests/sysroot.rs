use insta::assert_snapshot;
mod utils;

#[test]
fn test_sysroot() {
    let (out, err) = utils::run_upgrader("sysroot", "0.3.0", true);
    assert_snapshot!(out);
    assert_snapshot!(err);
}
