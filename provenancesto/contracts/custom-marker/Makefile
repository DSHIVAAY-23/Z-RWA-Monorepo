.PHONY: all
all: fmt build test lint schema optimize

.PHONY: fmt
fmt:
	@cargo fmt --all -- --check

.PHONY: build
build:
	@cargo build --release --target wasm32-unknown-unknown --lib

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
		-v "$(CURDIR)/../custom-marker":/custom-marker:Z \
		-v "$(CURDIR)/../price-feed":/price-feed:Z \
		-v "$(CURDIR)/../../packages":/packages:Z \
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
	
.PHONY: add_sub_admins
add_sub_admins:
	@scripts/testnet/execute/addSubAdmin.sh

.PHONY: create
create:
	@scripts/testnet/execute/createMarker.sh

.PHONY: set_whitelist
set_whitelist:
	@scripts/testnet/execute/setWhitelist.sh
	
.PHONY: mint_to
mint_to:
	@scripts/testnet/execute/MintTo.sh

.PHONY: update_token_limit
update_token_limit:
	@scripts/testnet/execute/updateTokenLimit.sh

.PHONY: get_balance
get_balance:
	@scripts/testnet/query/getBalance.sh
	
.PHONY: get_sub_admins
get_sub_admins:
	@scripts/testnet/query/getSubAdmins.sh
	
.PHONY: get_denom_config
get_denom_config:
	@scripts/testnet/query/getDenomConfig.sh

.PHONY: get_frozen_balance
get_frozen_balance:
	@scripts/testnet/query/getFrozenBalance.sh

.PHONY: get_circulating_supply
get_circulating_supply:
	@scripts/testnet/query/getCirculatingSupply.sh

.PHONY: get_by_denom
get_by_denom:
	@scripts/testnet/query/getByDenom.sh
