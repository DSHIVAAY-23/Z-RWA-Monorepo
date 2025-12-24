
provenanced query wasm contract-state smart tp15d2kxfntk3u8wtr42nsrgrtqf6jxf8lsn9qpj69nzkxh8ykhwfsq863kuz \
	'{
    "get_rate": {
        "symbol": "BAND"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
