provenanced tx wasm execute \
    tp1grngp33uvra9zwzv5aemsneul0c5j7y5jsfkltsxrgalcdf3gh0q7r2mn3 \
    '{
    "manage_agent": {
        "denom": "MCustomMarker",
        "update_type": {
            "add": "tp1esdv3xcal3fmc5aq93wvsujgkvc75mnf09jc73"
        }
    }
}' \
    --from $feebucket \
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