fmt:
	cargo fmt --all

lint:
	cargo clippy --workspace

test:
	cargo test --workspace

check:
	cargo fmt --all --check
	cargo clippy --workspace
	cargo test --workspace