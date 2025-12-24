
provenanced tx wasm execute \
    tp17xethxxpfq6rtg6wwzupkfmrc8mrt0k2s4c6nw30v90anlhx9mrqjndeec \
    '{
    "approve_request": {
        "request_id": "0x000000000000000000000004d6d0bb16fdcabb5b4e1985057192760abb68f18d",
        "request_type": "mint"
    }
}' \
    --from $dev \
    --amount 1vspn \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-axl-devnet-1 \
    --gas auto \
    --gas-prices 0vspn \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 |  jq '.txhash, .raw_log'

