
provenanced tx wasm execute \
    tp17xethxxpfq6rtg6wwzupkfmrc8mrt0k2s4c6nw30v90anlhx9mrqjndeec \
    '{
    "update_dest_config": {
        "config": {
            "chain": "onyx",
            "address": "0x466Ca44aC65567F87e3D1E69e0D16E2716a37B58"
        }
    }
}' \
    --from $dev \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-axl-devnet-1 \
    --gas auto \
    --gas-prices 0vspn \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq

