
provenanced query wasm contract-state smart tp1qfqq9tz0s7f57dwmd7zmxwhwvjcqvcwv5y7uvrr7xvkwc9uy68wqch75zx \
	'{
    "get_request_of": {
        "request_id": "0x0000000000000000000000052cb919048e3180356546298267947a122bb3f7d2"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
