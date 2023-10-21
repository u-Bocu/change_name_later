# Build

build_debug:
	cargo build --bin pirate_parrot_debug

build_service:
	cargo build --bin pirate_parrot_service

run_debug:
	cargo run --bin pirate_parrot_debug