
provenanced query wasm contract-state smart tp1ysjq08nlz4jh06spcfmv3fyewc2lrx80ldtlzw66rs4pnmnygl0svjmt5m \
	'{
    "get_burn_balance_of": {
        "owner": "tp1zu5rdmpk08epmlt4j6qejwgej203zz86thfns2"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
