env ?= testnet
# env ?= devnet

.PHONY: all
all: fmt build test lint schema optimize

.PHONY: fmt
fmt:
	@cargo build --release --target wasm32-unknown-unknown --lib

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
		--mount type=volume,source=marker_cache,target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/rust-optimizer:0.17.0
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

.PHONY: create
create:
	@scripts/${env}/execute/createMarker.sh
	
.PHONY: mint_to
mint_to:
	@scripts/${env}/execute/MintTo.sh

.PHONY: update_token_limit
update_token_limit:
	@scripts/${env}/execute/updateTokenLimit.sh

receive_cosmos:
	@scripts/devnet/execute/receiveMessageCosmos.sh

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

get_res:
	@scripts/${env}/query/getIBCResponse.sh

get_msg:
	@scripts/${env}/query/getStoredMessage.sh

get_freezed_accounts:
	@scripts/${env}/query/getFreezedAccounts.sh
