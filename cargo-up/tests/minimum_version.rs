use insta::assert_snapshot;
mod utils;

#[test]
fn test_minimum_version() {
    let (out, _) = utils::run(
        "../fixtures/rename_member",
        &[
            "up",
            "dep",
            "upgradee",
            "--lib-path",
            "../../cargo-up",
            "--path",
            "../upgrader",
            "--name",
            "upgrader",
            "--to-version",
            "0.3.0",
        ],
    );
    assert_snapshot!(out);
}
