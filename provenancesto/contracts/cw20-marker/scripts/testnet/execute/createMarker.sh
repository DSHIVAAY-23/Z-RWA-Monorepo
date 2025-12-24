
provenanced tx wasm execute \
    tp10m7er24gc7u0fl26qpm4d487d90vug2gw2s4kq9r5zw00nd4hymqgmrpa3 \
    '{
    "create": {
        "params": {
            "denom": "TJTest-12",
            "id": "unique",
            "issuer": "tp1ss30sspkkjnns8q95cuw83lrt8h8nvehpwf479",
            "tokenization_agent": "tp1r79n5nz7h6wnltv4g6lt3cek3h6d39p7jasv2k",
            "transfer_agent": "tp1zry40eq8cm2nt85uac2eagd58xuj34c4sxuhx6",
            "holding_period": "0"
        }
    }
}' \
    --from $minter \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 50000nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq '.txhash, .code, .raw_log'
