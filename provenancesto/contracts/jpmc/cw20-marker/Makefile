# env ?= testnet
env ?= devnet

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
	@docker run --rm -v $(CURDIR):/code:Z -v "$(CURDIR)/../../packages":/packages:Z \
		--mount type=volume,source=cw20_marker_cache,target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/rust-optimizer-arm64:0.14.0

deploy:
	@scripts/${env}/deploy/deploy.sh

init:
	@scripts/${env}/init/init.sh

migrate:
	@scripts/${env}/migrate/migrate.sh
	
add_sub_admins:
	@scripts/${env}/execute/addSubAdmin.sh

mint:
	@scripts/${env}/execute/mint.sh

burn:
	@scripts/${env}/execute/burn.sh	

mint_to:
	@scripts/${env}/execute/mintTo.sh

burn_from:
	@scripts/${env}/execute/burnFrom.sh	

transfer_from:
	@scripts/${env}/execute/transferFrom.sh	

approve:
	@scripts/${env}/execute/approve.sh	

get_name:
	@scripts/${env}/query/getName.sh

get_symbol:
	@scripts/${env}/query/getSymbol.sh

get_decimals:
	@scripts/${env}/query/getDecimals.sh

get_supply:
	@scripts/${env}/query/getTotalSupply.sh

get_balance:
	@scripts/${env}/query/getBalanceOf.sh
	
get_allowance:
	@scripts/${env}/query/getAllowance.sh

get_sub_admins:
	@scripts/${env}/query/getSubAdmins.sh

get_frozen_balance:
	@scripts/${env}/query/getFrozenBalance.sh

get_circulating_supply:
	@scripts/${env}/query/getCirculatingSupply.sh

get_by_denom:
	@scripts/${env}/query/getByDenom.sh

get_freezed_accounts:
	@scripts/${env}/query/getFreezedAccounts.sh
