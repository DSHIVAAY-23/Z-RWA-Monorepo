
provenanced query wasm contract-state smart tp1ysjq08nlz4jh06spcfmv3fyewc2lrx80ldtlzw66rs4pnmnygl0svjmt5m \
	'{
    "get_request_of": {
        "request_id": "0xa8c7a3a541fd0074e90a2fbef851abbaf21a93e22f28db4c9ed375cd6b41b96a"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
