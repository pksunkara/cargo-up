use insta::assert_snapshot;
use serial_test::serial;
mod utils;

#[test]
#[serial]
fn test_hook_on() {
    let (out, err) = utils::run_upgrader("hook_on", "0.6.0", true);
    assert_snapshot!(out);
    assert_snapshot!(err);
}
