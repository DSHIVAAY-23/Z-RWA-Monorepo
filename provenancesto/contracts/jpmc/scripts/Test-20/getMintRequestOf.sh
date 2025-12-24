
provenanced query wasm contract-state smart tp1we9chrq58nj3etdw3yss7ytqeaeeka5725u2m0lg7vg5yzrwa8usp8shgj \
	'{
    "get_request_of": {
        "request_id": "0xa8c7a3a541fd0074e90a2fbef851abbaf21a93e22f28db4c9ed375cd6b41b96a"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
