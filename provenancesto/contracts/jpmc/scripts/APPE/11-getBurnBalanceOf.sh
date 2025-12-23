
provenanced query wasm contract-state smart tp1xgzc3t3x3jshwfst83umkxthckh0dyqexa54dsuyyh00jsgs7tlse0a9ty \
	'{
    "get_burn_balance_of": {
        "owner": "tp1zu5rdmpk08epmlt4j6qejwgej203zz86thfns2"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
