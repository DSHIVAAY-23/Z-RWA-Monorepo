
provenanced query wasm contract-state smart tp12l00swtsf2ygy3l0zuf9jaqr4vzedy8trlmpfmr9cymjngc2ey9syex6qe \
	'{
    "get_request_allowances": {
        "owner": "tp1zu5rdmpk08epmlt4j6qejwgej203zz86thfns2",
        "spender": "tp1d8lzewx67da62k4ax5gcz4h90w236gnehfhx65y5ly24zwgdcyuscc48wx",
        "request_type": "mint"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
