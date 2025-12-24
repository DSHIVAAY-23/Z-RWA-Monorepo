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
	@docker run --rm --platform linux/amd64 -v $(CURDIR):/code:Z -v "$(CURDIR)/../../packages":/packages:Z \
		--mount type=volume,source=marker_cache,target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/rust-optimizer-arm64:0.14.0

.PHONY: deploy
deploy:
	@scripts/deploy/deploy.sh

.PHONY: init
init:
	@scripts/init/init.sh

.PHONY: migrate
migrate:
	@scripts/migrate/migrate.sh
	
.PHONY: add_sub_admins
add_sub_admins:
	@scripts/execute/addSubAdmin.sh

.PHONY: create
create:
	@scripts/execute/createMarker.sh

.PHONY: set_whitelist
set_whitelist:
	@scripts/execute/setWhitelist.sh
	
.PHONY: mint_to
mint_to:
	@scripts/execute/MintTo.sh

.PHONY: update_token_limit
update_token_limit:
	@scripts/execute/updateTokenLimit.sh

.PHONY: get_balance
get_balance:
	@scripts/query/getBalance.sh
	
.PHONY: get_sub_admins
get_sub_admins:
	@scripts/query/getSubAdmins.sh
	
.PHONY: get_denom_config
get_denom_config:
	@scripts/query/getDenomConfig.sh

.PHONY: get_frozen_balance
get_frozen_balance:
	@scripts/query/getFrozenBalance.sh

.PHONY: get_circulating_supply
get_circulating_supply:
	@scripts/query/getCirculatingSupply.sh

.PHONY: get_by_denom
get_by_denom:
	@scripts/query/getByDenom.sh
