
provenanced query wasm contract-state smart tp17xethxxpfq6rtg6wwzupkfmrc8mrt0k2s4c6nw30v90anlhx9mrqjndeec \
	'{
    "get_request_of": {
        "request_id": "0x000000000000000000000004d6d0bb16fdcabb5b4e1985057192760abb68f18d"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
