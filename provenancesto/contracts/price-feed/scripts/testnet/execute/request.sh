
provenanced tx wasm execute \
    tp15d2kxfntk3u8wtr42nsrgrtqf6jxf8lsn9qpj69nzkxh8ykhwfsq863kuz \
    '{
    "request": {
        "symbols": [
            "BTC",
            "ETH",
            "BAND"
        ]
    }
}' \
    --from $validator \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 26905nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq

