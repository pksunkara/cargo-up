use insta::assert_snapshot;
mod utils;

#[test]
fn test_rename_members() {
    let (out, err) = utils::run_upgrader("rename_members", "0.3.0", true);
    assert_snapshot!(out);
    assert_snapshot!(err);
}

// TODO: Member of member in Struct or Mmeber of variant in Enum
