use cargo_up::{Runner, Version};

pub fn runner() -> Runner {
    Runner::new()
        .minimum("0.2.0")
        .unwrap()
        .version(
            Version::new("0.3.0")
                .unwrap()
                .rename_members("upgradee::Struct", &[["member", "new_name"]])
                .rename_methods("upgradee::Struct", &[["print", "print_err"]]),
        )
        .version(
            Version::new("0.4.0")
                .unwrap()
                .rename_methods("structopt::StructOpt", &[["from_args", "parse"]]),
        )
}
