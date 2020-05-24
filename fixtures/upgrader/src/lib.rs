use cargo_up::{Runner, Version};

pub fn runner() -> Runner {
    Runner::new()
        .minimum("0.2.0")
        .unwrap()
        .version(
            Version::new("0.3.0")
                .unwrap()
                .rename_members("clap::errors::Error", &[["message", "cause"]]),
        )
        .version(
            Version::new("0.4.0")
                .unwrap()
                .rename_methods("structopt::StructOpt", &[["from_args", "parse"]]),
        )
}
