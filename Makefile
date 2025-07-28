.PHONY: build test check fmt clippy db-migrate db-reset db-setup docs-build docs-serve docs-clean docs-api publish-check ci dev-setup full-check

# Development tasks
build:
	cargo build --workspace

test:
	cargo test --workspace

test-integration:
	cargo test --package mimir-dm-llm --test main

test-all: test test-integration

check:
	cargo check --workspace

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace -- -D warnings

# Database tasks
db-migrate:
	cd crates/mimir-dm-db && diesel migration run

db-reset:
	cd crates/mimir-dm-db && diesel database reset

db-setup:
	cd crates/mimir-dm-db && diesel setup

# Documentation tasks
docs-build:
	cargo doc --workspace --no-deps
	cd docs && mdbook build
	cp -r target/doc docs/book/target/

docs-serve:
	cargo doc --workspace --no-deps
	cd docs && mdbook build
	cp -r target/doc docs/book/target/
	cd docs && mdbook serve --open

docs-clean:
	cd docs && mdbook clean
	rm -rf docs/book/target/

docs-api:
	cargo doc --workspace --no-deps --open

# Publishing tasks
publish-check:
	cargo publish --dry-run --workspace

# Combined workflows
ci: fmt clippy test build

dev-setup: db-setup docs-build

full-check: fmt clippy test docs-build