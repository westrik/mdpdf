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

.PHONY: release
release:
	@[ "$(V)" ] || { echo "Usage: make release V=0.1.7"; exit 1; }
	node -e "var p=require('./package.json');p.version='$(V)';require('fs').writeFileSync('package.json',JSON.stringify(p,null,2)+'\n')"
	sed -i '' 's/^version = ".*"/version = "$(V)"/' Cargo.toml
	@echo "Bumped to $(V). Edit CHANGELOG.md, then:"
	@echo "  git add package.json Cargo.toml CHANGELOG.md && git commit -m 'v$(V)' && git push"

