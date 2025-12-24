
provenanced query wasm contract-state smart tp1ysjq08nlz4jh06spcfmv3fyewc2lrx80ldtlzw66rs4pnmnygl0svjmt5m \
	'{
    "get_total_supply": {}
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
