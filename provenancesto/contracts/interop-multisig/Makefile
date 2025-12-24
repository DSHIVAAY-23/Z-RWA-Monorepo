.PHONY: all
all: fmt build test lint schema optimize

.PHONY: fmt
fmt:
	@cargo fmt --all -- --check

.PHONY: build
build:
	@cargo wasm

.PHONY: test
test:
	@cargo test

.PHONY: lint
lint:
	@cargo clippy -- -D warnings

.PHONY: schema
schema:
	@cargo schema

.PHONY: clean
clean:
	@cargo clean
	@cargo clean --target-dir artifacts

.PHONY: check
check:
	@cargo check

.PHONY: optimize
optimize:
	@docker run --rm --platform linux/amd64 -v $(CURDIR):/code:Z \
		-v "$(CURDIR)/../../packages":/packages:Z \
		-v "$(CURDIR)/../interop-core":/interop-core:Z \
		-v "$(CURDIR)/../cw20-marker":/cw20-marker:Z \
		--mount type=volume,source=marker_cache,target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/rust-optimizer:0.17.0

.PHONY: deploy
deploy:
	@scripts/testnet/deploy/deploy.sh

.PHONY: init
init:
	@scripts/testnet/init/init.sh

.PHONY: migrate
migrate:
	@scripts/testnet/migrate/migrate.sh
