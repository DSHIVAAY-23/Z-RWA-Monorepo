
provenanced tx wasm execute \
    tp12l00swtsf2ygy3l0zuf9jaqr4vzedy8trlmpfmr9cymjngc2ey9syex6qe \
    '{
    "approve_request": {
        "request_id": "0x22",
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

