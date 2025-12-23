
provenanced tx wasm execute \
    tp1d8lzewx67da62k4ax5gcz4h90w236gnehfhx65y5ly24zwgdcyuscc48wx \
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

