all: cli node

.PHONY: cli
cli:
	cargo build --release --features cli

.PHONY: node
node:
	corepack yarn build


.PHONY: test
test:
	cargo test
	CI=1 corepack yarn test

.PHONY: lint
lint:
	cargo fmt --check
	cargo clippy
	corepack yarn lint

.PHONY: fix-lint
fix-lint:
	cargo fmt
	cargo clippy --fix
	corepack yarn lint:fix

.PHONY: release
release:
	@[ "$(V)" ] || { echo "Usage: make release V=0.1.7"; exit 1; }
	node -e "var p=require('./package.json');p.version='$(V)';require('fs').writeFileSync('package.json',JSON.stringify(p,null,2)+'\n')"
	awk -v version="$(V)" '\
		/^\[package\]$$/ { in_package = 1 } \
		in_package && !updated && /^version = "/ { sub(/"[^"]*"$$/, "\"" version "\""); updated = 1 } \
		{ print } \
		END { if (!updated) exit 1 }' Cargo.toml > Cargo.toml.tmp && mv Cargo.toml.tmp Cargo.toml
	@echo "Bumped to $(V). Edit CHANGELOG.md, then:"
	@echo "  git add package.json Cargo.toml CHANGELOG.md && git commit -m 'v$(V)' && git push"
