use insta::assert_snapshot;
mod utils;

#[test]
fn test_package_not_found() {
    let err = utils::run_err("../fixtures/minimum_version/on", &["up", "dep", "notfound"]);
    assert_snapshot!(err);
}

#[test]
fn test_no_upgrader() {
    let err = utils::run_err("../fixtures/minimum_version/on", &["up", "dep", "upgradee"]);
    assert_snapshot!(err);
}

#[test]
fn test_bad_metadata() {
    let err = utils::run_err("../fixtures/bad_metadata/on", &["up", "dep", "clap"]);
    assert_snapshot!(err);
}
