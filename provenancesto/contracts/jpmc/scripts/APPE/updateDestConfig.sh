
provenanced tx wasm execute \
    tp1xgzc3t3x3jshwfst83umkxthckh0dyqexa54dsuyyh00jsgs7tlse0a9ty \
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

