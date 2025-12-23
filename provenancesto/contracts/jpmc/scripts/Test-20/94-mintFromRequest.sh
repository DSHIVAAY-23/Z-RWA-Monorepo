
provenanced tx wasm execute \
    tp1we9chrq58nj3etdw3yss7ytqeaeeka5725u2m0lg7vg5yzrwa8usp8shgj \
    '{
    "request_from": {
        "request_id": "0x494",
        "from": "tp1zu5rdmpk08epmlt4j6qejwgej203zz86thfns2",
        "amount": "1",
        "request_type": "mint"
    }
}' \
    --from $test \
    --amount 1vspn \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-axl-devnet-1 \
    --gas auto \
    --gas-prices 0vspn \
    --broadcast-mode block \
    --testnet \
    --yes \
	--output json \
	--node=http://34.70.126.95:26657 |  jq '.txhash, .code'

