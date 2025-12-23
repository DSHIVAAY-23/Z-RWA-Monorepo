
provenanced query wasm contract-state smart tp17xethxxpfq6rtg6wwzupkfmrc8mrt0k2s4c6nw30v90anlhx9mrqjndeec \
	'{
    "get_request_of": {
        "request_id": "0x1"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
