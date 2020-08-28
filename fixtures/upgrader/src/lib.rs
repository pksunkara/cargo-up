use cargo_up::{Runner, Version};

pub fn runner() -> Runner {
    Runner::new()
        .minimum("0.2.0")
        .unwrap()
        .version(
            Version::new("0.3.0")
                .unwrap()
                .rename_members("upgradee::Struct", &[["struct_member", "new_s_member"]])
                .rename_members("upgradee::Union", &[["union_member", "new_u_member"]])
                .rename_variants("upgradee::Enum", &[["Orange", "Pineapple"]])
                .rename_methods("upgradee::Struct", &[["print", "print_err"]])
                .rename_methods("upgradee::Enum", &[["talk", "talk_err"]])
                .rename_methods("upgradee::Union", &[["eat", "eat_err"]]),
        )
        .version(
            Version::new("0.4.0")
                .unwrap()
                .rename_methods("structopt::StructOpt", &[["from_args", "parse"]]),
        )
}
