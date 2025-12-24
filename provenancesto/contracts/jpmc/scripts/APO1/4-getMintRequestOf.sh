
provenanced query wasm contract-state smart tp1ysjq08nlz4jh06spcfmv3fyewc2lrx80ldtlzw66rs4pnmnygl0svjmt5m \
	'{
    "get_request_of": {
        "request_id": "0x000000000000000000000001f62e2fbb5d4e88d6ef98ba82feee215f1cc79047"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
