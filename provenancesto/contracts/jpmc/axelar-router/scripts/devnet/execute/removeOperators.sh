
provenanced tx wasm execute \
    tp1d8lzewx67da62k4ax5gcz4h90w236gnehfhx65y5ly24zwgdcyuscc48wx \
    '{
    "update_operators": {
        "update_type": {
            "remove": [
                "tp1s942m0d3km7hvnlxutdxc93dugyrnt7gyc8tv2v6a2z3cdk2c52qwnthyz"
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
