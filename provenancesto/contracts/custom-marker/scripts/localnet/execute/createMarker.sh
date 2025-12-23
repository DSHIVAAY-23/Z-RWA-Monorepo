
$local_prov/provenanced tx wasm execute \
    tp1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqf06p2p \
    '{
    "create": {
        "params": {
            "supply": "1000000",
            "denom": "CCustomMarker",
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
    --from $local_validator \
    --keyring-backend test \
    --home $local_prov_path \
    --chain-id testing \
    --gas 4000000 \
    --gas-prices 26905nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	| jq  
