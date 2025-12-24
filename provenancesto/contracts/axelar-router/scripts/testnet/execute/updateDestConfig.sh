
provenanced tx wasm execute \
    tp1wjea0da3kzt7rcddjyvf9gxf7nkvzuc89dkfrdh3ywqnz7kt6z5qxpkvxk \
    '{
    "update_dest_config": {
        "config": {
            "chain": "Polygon",
            "address": "0x6a14D924E0224Eb650122b4F1E27828b1B9eF58E"
        }
    }
}' \
    --from $validator \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 26905nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq '.txhash, .code, .raw_log'

