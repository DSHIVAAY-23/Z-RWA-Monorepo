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
		-v "$(CURDIR)/../cw20-marker":/cw20-marker:Z \
		--mount type=volume,source=marker_factory_cache,target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/rust-optimizer-arm64:0.14.0

.PHONY: deploy
deploy:
	@scripts/${env}/deploy/deploy.sh

.PHONY: init
init:
	@scripts/${env}/init/init.sh

.PHONY: migrate
migrate:
	@scripts/${env}/migrate/migrate.sh
	
.PHONY: add_sub_admins
add_sub_admins:
	@scripts/${env}/execute/addSubAdmin.sh
	
deploy_token:
	@scripts/${env}/execute/deployToken.sh

update_code:
	@scripts/${env}/execute/updateCode.sh

.PHONY: get_balance
get_balance:
	@scripts/${env}/query/getBalance.sh
	
.PHONY: get_sub_admins
get_sub_admins:
	@scripts/${env}/query/getSubAdmins.sh

.PHONY: get_frozen_balance
get_frozen_balance:
	@scripts/${env}/query/getFrozenBalance.sh

.PHONY: get_circulating_supply
get_circulating_supply:
	@scripts/${env}/query/getCirculatingSupply.sh

.PHONY: get_by_denom
get_by_denom:
	@scripts/${env}/query/getByDenom.sh

get_freezed_accounts:
	@scripts/${env}/query/getFreezedAccounts.sh

get_code_id:
	@scripts/${env}/query/getCodeId.sh

get_contract_by_denom:
	@scripts/${env}/query/getContractByDenom.sh

get_denom_by_contract:
	@scripts/${env}/query/getDenomByContract.sh