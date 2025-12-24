
provenanced query wasm contract-state smart tp1ucrz2ntndpxhdnz2hr55gqwjp3jpmpfrjucd2tvka60dq7rujklq0j0yq5 \
	'{
    "get_request_of": {
        "request_id": "0xa8c7a3a541fd0074e90a2fbef851abbaf21a93e22f28db4c9ed375cd6b41b96a"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
