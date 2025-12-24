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
	@cargo run --bin schema


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


deploy:
	@scripts/deploy/deploy.sh

init:
	@scripts/init/init.sh

migrate:
	@scripts/migrate/migrate.sh
	
add_sub_admins:
	@scripts/execute/addSubAdmin.sh
	
mint:
	@scripts/execute/mint.sh

burn:
	@scripts/execute/burn.sh

increase_allowance:
	@scripts/execute/increaseAllowance.sh

decrease_allowance:
	@scripts/execute/decreaseAllowance.sh

burn_from:
	@scripts/execute/burnFrom.sh

transfer:
	@scripts/execute/transfer.sh

transfer_from:
	@scripts/execute/transferFrom.sh

get_balance:
	@scripts/query/getBalance.sh
	
get_sub_admins:
	@scripts/query/getSubAdmins.sh

get_frozen_balance:
	@scripts/query/getFrozenBalance.sh

get_circulating_supply:
	@scripts/query/getCirculatingSupply.sh

get_token_info:
	@scripts/query/getTokenInfo.sh

get_allowance:
	@scripts/query/getAllowance.sh
