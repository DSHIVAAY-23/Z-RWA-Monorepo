
provenanced query wasm contract-state smart tp1jlefmscn0yfyg5fm2vwjxazwxxgtnp20dvx2fauvm7af2ucevvdqd3afls \
	'{
    "get_request_of": {
        "request_id": "0x000000000000000000000002f0cf63d9111cc72fa8e1dc2dc8a9d09f7c574bd6"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
