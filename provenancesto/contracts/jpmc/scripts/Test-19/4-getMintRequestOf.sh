
provenanced query wasm contract-state smart tp16t96gk6telazd5f9uvxynxdtzf0503vqw6fsvrkq2lxzg9vmc98qdyavdg \
	'{
    "get_request_of": {
        "request_id": "0x03591f662ea77dabbbefe99688fd04f23e214e0ad15a66304e9fa60754ca1d94"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
