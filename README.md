# cargo-up

Upgrade your dependencies by automatically fixing your code

**Dont be afraid to upgrade**

## Installation

```
cargo install cargo-up --features cli --no-default-features
```

## Users Workflow

Assuming that you have a project with the following `Cargo.toml`

```toml
[dependencies]
foo = "0.8.2"
```

If `foo` has released `0.9.0` with breaking changes along with a new release of their
`foo_up` which details the changes, you can simply run the following command in your
project:

```bash
cargo up dep foo
```

Your project code will be automatically upgraded to use the new `foo@0.9.0`.

**NOTE**: The tool upgrades to the latest version of the dependency, which means it can
do several sequential version upgrades one after the other in a single run.

## Maintainers Workflow

TODO:
