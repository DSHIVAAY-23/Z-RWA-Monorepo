
provenanced query wasm contract-state smart tp12l00swtsf2ygy3l0zuf9jaqr4vzedy8trlmpfmr9cymjngc2ey9syex6qe \
	'{
    "get_request_of": {
        "request_id": "0xc1d8f159b9543e0d4d0d855323b501eda5196e53951a03911fbdb4f65b01f539"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
