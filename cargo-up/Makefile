test:
	cargo insta test --review -j 1 --features cli

clap:
	cargo run --features cli up --manifest-path ../../up_test/Cargo.toml \
	dep clap \
	--name clap_up --to-version 3.0.0-beta.1 --lib-path . \
	--path ../../../clap-rs/clap/clap_up \
