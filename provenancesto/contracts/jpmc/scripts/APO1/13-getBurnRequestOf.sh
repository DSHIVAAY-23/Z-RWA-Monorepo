
provenanced query wasm contract-state smart tp1ysjq08nlz4jh06spcfmv3fyewc2lrx80ldtlzw66rs4pnmnygl0svjmt5m \
	'{
    "get_request_of": {
        "request_id": "0x0000000000000000000000048f518aedd5523820e4d9c1e55180ec1f75383485"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
