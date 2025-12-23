
provenanced tx wasm execute \
    tp1emy7mkz3p3kcq3vmxjacud2m255gutwykplyns7z6yemp6mnqppsvlfnup \
    '{
    "request": {
        "request_id": "0x3",
        "amount": "1500000000",
        "request_type": "burn"
    }
}' \
    --from $holding103 \
    --custom-denom OasisToken \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-axl-devnet-1 \
    --gas-prices 0OasisToken \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 |  jq '.txhash, .code' 
