
provenanced query wasm contract-state smart tp1wjea0da3kzt7rcddjyvf9gxf7nkvzuc89dkfrdh3ywqnz7kt6z5qxpkvxk \
	'{
    "get_stored_message": {}
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
