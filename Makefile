all: cli node

.PHONY: cli
cli:
	cargo build --release --features cli

.PHONY: node
node:
	yarn build


.PHONY: test
test:
	cargo test
	yarn test

.PHONY: lint
lint:
	cargo fmt --check
	cargo clippy
	yarn lint

.PHONY: fix-lint
fix-lint:
	cargo fmt
	cargo clippy --fix
	yarn lint:fix

