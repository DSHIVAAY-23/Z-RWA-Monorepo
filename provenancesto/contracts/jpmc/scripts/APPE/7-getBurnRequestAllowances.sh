
provenanced query wasm contract-state smart tp1xgzc3t3x3jshwfst83umkxthckh0dyqexa54dsuyyh00jsgs7tlse0a9ty \
	'{
    "get_request_allowances": {
        "owner": "tp1zu5rdmpk08epmlt4j6qejwgej203zz86thfns2",
        "spender": "tp1d8lzewx67da62k4ax5gcz4h90w236gnehfhx65y5ly24zwgdcyuscc48wx",
        "request_type": "burn"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
