
$local_prov/provenanced query wasm contract-state smart tp1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqf06p2p \
	'{
    "get_denom_config": {
        "denom": "CCustomMarker"
    }
}' \
    --testnet \
	--output json \
	| jq
