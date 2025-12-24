
provenanced query wasm contract-state smart tp1ucrz2ntndpxhdnz2hr55gqwjp3jpmpfrjucd2tvka60dq7rujklq0j0yq5 \
	'{
    "get_request_of": {
        "request_id": "0xc7c84f24f2ba8dbd477a02cdfeca57124d0a924826c2dca9dfc5c34e644300cd"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
