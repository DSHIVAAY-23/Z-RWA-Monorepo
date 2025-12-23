$local_prov/provenanced tx wasm execute \
    tp1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqf06p2p \
    '{
    "send": {
        "amount": "900",
        "denom": "CCustomMarker",
        "to": "tp1c7apkgl4l8pw72nsh6uvvr7d6fy3c98hyg0sct"
    }
}' \
    --from $tarun \
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
    