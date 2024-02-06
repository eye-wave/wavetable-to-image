#!make
.PHONY: build lint format

build:
	cargo build --release
	cp target/release/wavetable-to-image $$HOME/.local/bin
	du -h $$HOME/.local/bin/wavetable-to-image

lint:
	cargo clippy

format:
	cargo fmt
