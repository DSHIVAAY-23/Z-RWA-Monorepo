
provenanced query wasm contract-state smart tp1xgzc3t3x3jshwfst83umkxthckh0dyqexa54dsuyyh00jsgs7tlse0a9ty \
	'{
    "get_request_of": {
        "request_id": "0x000000000000000000000001a5b63558c8bbc8cccf2aea7a764eb8b4857c1bfa"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
