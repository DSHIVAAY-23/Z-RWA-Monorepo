
provenanced tx wasm execute \
    tp1we9chrq58nj3etdw3yss7ytqeaeeka5725u2m0lg7vg5yzrwa8usp8shgj \
    '{
    "manage_request_allowance": {
        "spender": "tp1ujwgh8hs0l235plvln6qd0jxgxjym92z22chn7",
        "update_type": {
            "add": "340282366920938463463374607431768211455"
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
	--node=http://34.70.126.95:26657 |  jq '.txhash, .code'

