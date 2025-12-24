
provenanced tx wasm execute \
    tp1emy7mkz3p3kcq3vmxjacud2m255gutwykplyns7z6yemp6mnqppsvlfnup \
    '{
    "approve_request": {
        "request_id": "0x3",
        "request_type": "burn"
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
	--node=http://34.70.126.95:26657 |  jq '.txhash, .code'
