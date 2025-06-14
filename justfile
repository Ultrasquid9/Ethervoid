set shell := ["nu", "-c"]

default: lint run

run:
	cargo run

fg:
	cargo flamegraph --dev

fg_release:
	CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph

update:
	git fetch
	git pull
	cargo update

alias fmt := lint

lint: stylua
	@echo '{{ style("command")}}stylua{{ NORMAL }}'
	cargo fmt
	cargo clippy

stylua:
	#!/usr/bin/env nu	
	for $file in (ls ...(glob **/*.lua)) {
		stylua $file.name
	}
