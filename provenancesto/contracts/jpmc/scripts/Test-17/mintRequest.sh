
provenanced tx wasm execute \
    tp1vmnntr773a5p7s4k0t39v6vcgcq87kq2zaw94cy85850n79jx2kqw4ky9f \
    '{
    "request": {
        "request_id": "0x1",
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
