
provenanced query wasm contract-state smart tp1ucrz2ntndpxhdnz2hr55gqwjp3jpmpfrjucd2tvka60dq7rujklq0j0yq5 \
	'{
    "get_request_of": {
        "request_id": "0x0000000000000000000000028f389fd332b039ae007d0e6dc6798e2a11480c46"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
