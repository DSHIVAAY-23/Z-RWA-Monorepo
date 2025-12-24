
provenanced tx wasm execute \
    tp12l00swtsf2ygy3l0zuf9jaqr4vzedy8trlmpfmr9cymjngc2ey9syex6qe \
    '{
    "request_from": {
        "request_id": "0x42",
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
    --gas-prices 0vspn \
    --broadcast-mode block \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 |  jq 

