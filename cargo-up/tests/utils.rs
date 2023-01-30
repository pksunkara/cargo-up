#![allow(unused)]
use assert_cmd::Command;
use insta::assert_snapshot;
use std::{
    fs::{copy, read_to_string, write},
    path::PathBuf,
    str::from_utf8,
};

pub fn run(dir: &str, args: &[&str]) -> (String, String) {
    let output = Command::cargo_bin("cargo-up")
        .unwrap()
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap();

    let out = from_utf8(&output.stdout).unwrap();
    let err = from_utf8(&output.stderr).unwrap();

    (out.to_string(), err.to_string())
}

pub fn run_out(dir: &str, args: &[&str]) -> String {
    let (out, err) = run(dir, args);

    assert!(err.is_empty());
    out
}

pub fn run_err(dir: &str, args: &[&str]) -> String {
    let (out, err) = run(dir, args);

    assert!(out.is_empty());
    err
}

pub fn run_upgrader(dir: &str, version: &str, generate: bool) -> (String, String) {
    let mut fixture_on = PathBuf::new();

    fixture_on.push("..");
    fixture_on.push("fixtures");

    if generate {
        let mut from = fixture_on.clone();
        from.push(dir);

        fixture_on.push("_run");
        let mut to = fixture_on.clone();

        from.push("on.rs");
        to.push("on");
        to.push("src");
        to.push("main.rs");

        copy(&from, &to).unwrap();

        from.pop();
        from.push("upgradee.rs");
        to.pop();
        to.pop();
        to.pop();
        to.push("upgradee");
        to.push("src");
        to.push("lib.rs");

        copy(&from, &to).unwrap();
    } else {
        fixture_on.push(dir);
    }

    fixture_on.push("on");

    let on = fixture_on.clone();

    fixture_on.push("src");
    fixture_on.push("main.rs");

    let original = read_to_string(&fixture_on).unwrap();

    let (out, err) = run(
        &on.to_string_lossy(),
        &[
            "up",
            "dep",
            "upgradee",
            "--lib-path",
            "../../../cargo-up",
            "--upgrader-path",
            "../../upgrader",
            "--upgrader-name",
            "upgrader",
            "--dep-version",
            version,
            "--suppress-cargo-output",
        ],
    );

    let file_content = read_to_string(&fixture_on).unwrap().replace("\r\n", "\n");
    assert_snapshot!(file_content);

    write(&fixture_on, original).unwrap();

    (out, err)
}
