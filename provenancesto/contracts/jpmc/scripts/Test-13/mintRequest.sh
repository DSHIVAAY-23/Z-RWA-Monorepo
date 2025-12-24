
provenanced tx wasm execute \
    tp12l00swtsf2ygy3l0zuf9jaqr4vzedy8trlmpfmr9cymjngc2ey9syex6qe \
    '{
    "request": {
        "request_id": "0x43",
        "amount": "1000",
        "request_type": "mint"
    }
}' \
    --from $user \
    --custom-denom OasisToken \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-axl-devnet-1 \
    --gas auto \
    --gas-prices 0OasisToken \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 |  jq '.raw_log'
