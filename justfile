default: lint run

lint:
	cargo fmt
	cargo clippy

run:
	cargo run

fg:
	cargo flamegraph --dev

update:
	git fetch
	git pull
	cargo update
