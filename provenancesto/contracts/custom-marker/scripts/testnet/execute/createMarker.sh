
provenanced tx wasm execute \
    tp16h50hcp3m777t68vv42x6kzdrym9dyn5ucxq6tpj46qnnye0k97slzkku3 \
    '{
    "create": {
        "params": {
            "denom": "MCustomMarker",
            "denom_config": {
                "token_limit": "10000",
                "country_codes": [
                    1,
                    91
                ]
            },
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
    --gas-prices 26905nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
