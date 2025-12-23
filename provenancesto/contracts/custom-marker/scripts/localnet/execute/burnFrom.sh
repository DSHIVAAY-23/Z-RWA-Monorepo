
$local_prov/provenanced tx wasm execute \
    tp1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqf06p2p \
    '{
    "burn_from": {
        "burn_from_params": [
            {
                "denom": "CCustomMarker",
                "mint_burn_data": [
                    {
                        "address": "tp1y49xa3cl0qsa7lr0vhasgsp6mnpcun8qc3n7nq",
                        "amount": "200"
                    }
                ]
            }
        ]
    }
}' \
    --from $local_validator \
    --keyring-backend test \
    --home $local_prov_path \
    --chain-id testing \
    --gas 4000000 \
    --gas-prices 1905nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	| jq
