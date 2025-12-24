
provenanced tx wasm execute \
    tp1d8lzewx67da62k4ax5gcz4h90w236gnehfhx65y5ly24zwgdcyuscc48wx \
    '{
    "update_operators": {
        "update_type": {
            "add": [
                "0xebf2bb22e15B29514C62034eDdcc372Bf6CddF31",
                "0x983D97001CC927e9ccf0Fc214f5DCf89c850948a"
            ]
        }
    }
}' \
    --from $dev \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-axl-devnet-1 \
    --gas 4000000 \
    --gas-prices 0vspn \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
