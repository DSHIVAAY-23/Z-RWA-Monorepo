provenanced tx wasm migrate \
    tp17xethxxpfq6rtg6wwzupkfmrc8mrt0k2s4c6nw30v90anlhx9mrqjndeec \
    90 \
    '{}' \
    --from "$dev" \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-axl-devnet-1 \
    --broadcast-mode block \
    --testnet \
    --yes \
    --gas-prices 0vspn \
    --node=http://34.70.126.95:26657 \
	--output json | jq

