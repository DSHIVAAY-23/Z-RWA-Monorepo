provenanced tx wasm migrate \
    tp15d2kxfntk3u8wtr42nsrgrtqf6jxf8lsn9qpj69nzkxh8ykhwfsq863kuz \
    467 \
    '{}' \
    --from "$feebucket" \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --broadcast-mode block \
    --testnet \
    --yes \
    --gas 4000000 \
    --gas-prices 1905nhash \
    --node=https://rpc.test.provenance.io:443 \
	--output json | jq

