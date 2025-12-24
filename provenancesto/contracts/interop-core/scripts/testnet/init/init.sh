provenanced tx wasm instantiate 691 \
	'{
    "multi_sig": "tp1hhw47wfzlvsqa08wv2m3ntgpu3hfpjneyxsfypzwz4xz956ta88quhhsv8",
    "deployed_chain": "Provenance"
}' \
    --admin "$feebucket" \
    --label interop-core \
    --from "$feebucket" \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 300000nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq '.txhash, .code, .raw_log'
    
    


