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
		-v "$(CURDIR)/../../packages":/packages:Z \
		--mount type=volume,source=marker_cache,target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/rust-optimizer:0.17.0

.PHONY: deploy
deploy:
	@scripts/deploy/deploy.sh

.PHONY: init
init:
	@scripts/init/init.sh

.PHONY: migrate
migrate:
	@scripts/migrate/migrate.sh

.PHONY: add_admin
add_admin:
	@scripts/execute/addAdmin.sh

.PHONY: remove_admin
remove_admin:
	@scripts/execute/removeAdmin.sh

.PHONY: create
create:
	@scripts/execute/create.sh

.PHONY: share_stable_coin
share_stable_coin:
	@scripts/execute/shareStableCoin.sh

.PHONY: send_stable_coins
send_stable_coins:
	@scripts/execute/sendStableCoins.sh

.PHONY: add_agent
add_agent:
	@scripts/execute/addAgent.sh

.PHONY: remove_agent
remove_agent:
	@scripts/execute/removeAgent.sh

.PHONY: get_admins
get_admins:
	@scripts/query/getAdmins.sh

.PHONY: get_agent
get_agent:
	@scripts/query/getAgentByDenom.sh

.PHONY: get_payments
get_payments:
	@scripts/query/getPayments.sh
