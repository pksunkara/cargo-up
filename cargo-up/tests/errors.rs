use insta::assert_snapshot;
mod utils;

#[test]
fn test_package_not_found() {
    let err = utils::run_err("../fixtures/rename_member/on", &["up", "dep", "not-found"]);
    assert_snapshot!(err);
}

#[test]
fn test_no_upgrader() {
    let err = utils::run_err("../fixtures/rename_member/on", &["up", "dep", "upgradee"]);
    assert_snapshot!(err);
}
