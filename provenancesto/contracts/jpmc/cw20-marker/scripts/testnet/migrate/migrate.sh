provenanced tx wasm migrate \
    tp1h7scjsemhcy9pgs9ges488n5cxdghz4sj2qk0qwgrjw7jrpgvets8j4tzq \
    603 \
    '{}' \
    --from "$feebucket" \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --broadcast-mode block \
    --testnet \
    --yes \
    --gas auto \
    --gas-prices 26905nhash \
    --node=https://rpc.test.provenance.io:443 \
	--output json | jq

