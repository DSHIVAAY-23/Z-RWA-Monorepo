
provenanced query wasm contract-state smart tp1ysjq08nlz4jh06spcfmv3fyewc2lrx80ldtlzw66rs4pnmnygl0svjmt5m \
	'{
    "get_request_of": {
        "request_id": "0xc7c84f24f2ba8dbd477a02cdfeca57124d0a924826c2dca9dfc5c34e644300cd"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
