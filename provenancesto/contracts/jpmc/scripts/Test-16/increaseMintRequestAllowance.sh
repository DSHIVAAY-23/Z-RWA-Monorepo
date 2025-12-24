
provenanced tx wasm execute \
    tp1lu8mr2ge4hz7kd5qky2ysayyrtrgst7yslqd5ekuuqp79m9g9k0qwdeq54 \
    '{
    "manage_request_allowance": {
        "spender": "tp1ujwgh8hs0l235plvln6qd0jxgxjym92z22chn7",
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

