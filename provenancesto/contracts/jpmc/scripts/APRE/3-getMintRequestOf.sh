
provenanced query wasm contract-state smart tp15vaux4uwcehrkh5t6rhjctkr8z05tmpm9hc5hrgzeu0xxy2gvtxqhdlhyz \
	'{
    "get_request_of": {
        "request_id": "0x000000000000000000000004d6d0bb16fdcabb5b4e1985057192760abb68f18d"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
