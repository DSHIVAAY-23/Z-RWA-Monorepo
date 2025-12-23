
provenanced query wasm contract-state smart tp16t96gk6telazd5f9uvxynxdtzf0503vqw6fsvrkq2lxzg9vmc98qdyavdg \
	'{
    "get_request_of": {
        "request_id": "0x572529a6ec6219ae60d716a6ffa3b2109d94e3d7763fc3fa833abc6e62b0f70b"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
