use cargo_up::{
    anyhow::{bail, Result},
    semver::Version as SemverVersion,
    Runner, Upgrader, Version,
};

fn init(_: &mut Upgrader, from: &SemverVersion) -> Result<()> {
    if from.to_string() == "0.4.0" {
        bail!("Can't upgrade from 0.4.0");
    }

    Ok(())
}

pub fn runner() -> Runner {
    Runner::new()
        .minimum("0.2.0")
        .unwrap()
        .version(
            Version::new("0.3.0")
                .unwrap()
                .rename_structs("upgradee", &[["OldStruct", "NewStruct"]])
                .rename_members("upgradee::Struct", &[["struct_member", "new_s_member"]])
                .rename_members("upgradee::Union", &[["union_member", "new_u_member"]])
                .rename_variants(
                    "upgradee::Enum",
                    &[
                        ["Orange", "Pineapple"],
                        ["Grape", "Berry"],
                        ["Melon", "Papaya"],
                    ],
                )
                .rename_methods("upgradee::Struct", &[["print", "print_err"]])
                .rename_methods("upgradee::Enum", &[["talk", "talk_err"]])
                .rename_methods("upgradee::Union", &[["eat", "eat_err"]]),
        )
        .version(
            Version::new("0.4.0")
                .unwrap()
                .rename_methods("structopt::StructOpt", &[["from_args", "parse"]]),
        )
        .version(Version::new("0.5.0").unwrap().init(init))
}
