
provenanced query wasm contract-state smart tp1we9chrq58nj3etdw3yss7ytqeaeeka5725u2m0lg7vg5yzrwa8usp8shgj \
	'{
    "get_request_of": {
        "request_id": "0x216"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
