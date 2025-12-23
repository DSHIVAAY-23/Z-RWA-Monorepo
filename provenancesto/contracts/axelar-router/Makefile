# env ?= devnet
env ?= testnet

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
		--mount type=volume,source=axelar_router_cache,target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/rust-optimizer-arm64:0.14.0

deploy:
	@scripts/${env}/deploy/deploy.sh

init:
	@scripts/${env}/init/init.sh

migrate:
	@scripts/${env}/migrate/migrate.sh
	
send_evm:
	@scripts/${env}/execute/sendMsgEvm.sh

send_cosmos:
	@scripts/${env}/execute/sendMsgCosmos.sh

upd_dest:
	@scripts/${env}/execute/updateDestinationAddress.sh

get_msg:
	@scripts/${env}/query/getStoredMessage.sh

get_res:
	@scripts/${env}/query/getIBCResponse.sh

get_dest_addr:
	@scripts/${env}/query/getDestAddr.sh
