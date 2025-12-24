
provenanced query wasm contract-state smart tp1grngp33uvra9zwzv5aemsneul0c5j7y5jsfkltsxrgalcdf3gh0q7r2mn3 \
	'{
    "get_agent_by_denom": {
        "denom": "MCustomMarker"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq