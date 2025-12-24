
provenanced tx wasm execute \
    tp1vmnntr773a5p7s4k0t39v6vcgcq87kq2zaw94cy85850n79jx2kqw4ky9f \
    '{
    "manage_request_allowance": {
        "spender": "tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp",
        "update_type": {
            "add": "10000000"
        },
        "request_type": "mint"
    }
}' \
    --from $user \
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

