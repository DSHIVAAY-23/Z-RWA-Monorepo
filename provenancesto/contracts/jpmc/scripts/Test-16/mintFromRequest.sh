
provenanced tx wasm execute \
    tp1lu8mr2ge4hz7kd5qky2ysayyrtrgst7yslqd5ekuuqp79m9g9k0qwdeq54 \
    '{
    "request_from": {
        "request_id": "0x140",
        "from": "tp1zu5rdmpk08epmlt4j6qejwgej203zz86thfns2",
        "amount": "1",
        "request_type": "mint"
    }
}' \
    --from $dev \
    --amount 1vspn \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-axl-devnet-1 \
    --gas auto \
    --gas-prices 0vspn \
    --yes \
    --broadcast-mode block \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 |  jq 

