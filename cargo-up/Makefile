test:
	cargo insta test --review -j 1 --features cli --no-default-features

clap:
	cargo run --features cli --no-default-features up --manifest-path ../../../clap-rs/v2/Cargo.toml \
	dep clap \
	--name clap_up --to-version 3.0.0-rc.0 --lib-path . \
	--path ../../../clap-rs/clap/clap_up \

run:
	cargo run --features cli --no-default-features up --manifest-path ../fixtures/_run/on/Cargo.toml \
	dep upgradee \
	--name upgrader --to-version 0.3.0 --lib-path . \
	--path ../fixtures/upgrader \

testcase:
	cargo run --features cli --no-default-features up --manifest-path ../fixtures/$(case)/on/Cargo.toml \
	dep upgradee \
	--name upgrader --to-version $(version) --lib-path . \
	--path ../fixtures/upgrader \
