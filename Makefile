all: fmt clippy run

fmt:
	rustup run nightly cargo fmt -- --check

clippy:
	rustup run nightly cargo clippy -- -D warnings

run:
	cargo run
