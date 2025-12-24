
provenanced query wasm contract-state smart tp1vmnntr773a5p7s4k0t39v6vcgcq87kq2zaw94cy85850n79jx2kqw4ky9f \
	'{
    "get_request_of": {
        "request_id": "0x1"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
