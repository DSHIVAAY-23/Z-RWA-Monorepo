provenanced tx wasm execute \
    tp1grngp33uvra9zwzv5aemsneul0c5j7y5jsfkltsxrgalcdf3gh0q7r2mn3 \
    '{
    "distribute_and_burn": {
        "denom": "MCustomMarker",
        "coin_type": "usdt",
        "distributions": [
            {
                "investor": "tp1clq0rtxlwtg384mtsx2333hv75nn0va8gw8md5",
                "amount": "4",
                "token": "12"
            }
        ]
    }
}' \
    --from $feebucket \
    --amount 4USDT-Test \
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