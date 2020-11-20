mod utils;

#[test]
fn test_bad_metadata() {
    let err = utils::run_err("../fixtures/bad_metadata/on", &["up", "dep", "clap"]);
    assert!(err
        .find("Error during execution of `cargo metadata`:     Updating crates.io index\nerror: failed to select a version for the requirement `clap = \"^0.3.8\"`")
        .is_some());
}
