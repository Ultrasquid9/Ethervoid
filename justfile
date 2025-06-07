default: lint run

lint:
	cargo fmt
	cargo clippy

run:
	cargo run

fg:
	cargo flamegraph --dev
