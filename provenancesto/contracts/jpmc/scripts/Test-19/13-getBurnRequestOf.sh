
provenanced query wasm contract-state smart tp16t96gk6telazd5f9uvxynxdtzf0503vqw6fsvrkq2lxzg9vmc98qdyavdg \
	'{
    "get_request_of": {
        "request_id": "0x14f281554bd4a6279db8fddbc06cb37d8a70b5559774f57ca697807085198ee4"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
