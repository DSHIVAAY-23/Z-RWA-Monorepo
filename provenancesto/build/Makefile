.PHONY: all start build-store init
BLOCKCHAIN_PID_FILE := blockchain_process.pid
CONTRACT_PID_FILE := contrcat_process.pid
REPO_URL = https://github.com/provenance-io/provenance
BRANCH = main
CONTRACT_REPO_URL = https://github.com/Oasis-Pro-Inc/provenancesto
CONTRACT_BRANCH = development
export CODE_ID := my-code-id
export NAME := name
DENOM ?= nhash
MIN_FLOOR_PRICE ?= 1905
CHAIN_ID ?= testing
BUILDDIR ?= ${PWD}/build

start:
	@if test -f $(BLOCKCHAIN_PID_FILE); then \
		echo "Process already running with PID $$(cat $(BLOCKCHAIN_PID_FILE))" ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced start ; \
	else \
		echo "Starting process..." ; \
		./myprocess & echo $$! > $(BLOCKCHAIN_PID_FILE) ; \
		mkdir provnode ; \
		cd provnode ; \
		git clone --depth=1 --branch=$(BRANCH) $(REPO_URL) ; \
		cd provenance ; \
		make install ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced init --chain-id=$(CHAIN_ID) testing --custom-denom=$(DENOM); \
		provenanced -t --home $(BUILDDIR)/run/provenanced keys add validator --keyring-backend test ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced keys add bob --keyring-backend test ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-root-name validator pio --keyring-backend test ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-root-name validator pb --restrict=false --keyring-backend test ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-root-name validator io --restrict --keyring-backend test ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-root-name validator provenance --keyring-backend test ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-account validator 100000000000000000000$(DENOM)  --keyring-backend test ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-account bob 100000000000000000000hotdogcoin  --keyring-backend test ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced gentx validator 1000000000000000$(DENOM)  --keyring-backend test --chain-id=$(CHAIN_ID); \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-marker 100000000000000000000$(DENOM)  --manager validator --access mint,burn,admin,withdraw,deposit --activate --keyring-backend test; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-marker 100000000000000000000hotdogcoin  --manager bob --access mint,burn,admin,withdraw,deposit --activate --keyring-backend test; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-msg-fee /provenance.name.v1.MsgBindNameRequest 10000000000$(DENOM) ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-msg-fee /provenance.marker.v1.MsgAddMarkerRequest 100000000000$(DENOM) ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-msg-fee /provenance.attribute.v1.MsgAddAttributeRequest 10000000000$(DENOM) ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-msg-fee /provenance.metadata.v1.MsgWriteScopeRequest 10000000000$(DENOM) ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-custom-floor $(MIN_FLOOR_PRICE)$(DENOM) ; \
		cat $(BUILDDIR)/run/provenanced/config/genesis.json | jq '.app_state.auth.accounts[3].allow_forced_transfer = true' > tmp.json ; \
		mv tmp.json $(BUILDDIR)/run/provenanced/config/genesis.json ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced collect-gentxs ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced config set minimum-gas-prices ${MIN_FLOOR_PRICE}${DENOM} ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced start ; \
	fi


store:
	provenanced -t --home $(BUILDDIR)/run/provenanced tx wasm store ${PWD}/contracts/custom_marker/target/wasm32-unknown-unknown/release/custom_marker.wasm --from validator --chain-id=$(CHAIN_ID) --broadcast-mode block -y --gas 4000000 --gas-prices 1905nhash -o json > ${PWD}/wasm_storeInfo.json ; \



init:
	provenanced -t --home $(BUILDDIR)/run/provenanced tx wasm instantiate $(CODE_ID) '{"name":"$(NAME)"}' --label "custom-marker" --gas 4000000 --gas-prices 4500nhash --from validator --chain-id=$(CHAIN_ID) -t  --broadcast-mode block -y --no-admin -o json> ${PWD}/contract_info.json ; \


