
provenanced query wasm contract-state smart tp1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqf06p2p \
	'{
    "get_hold_period": {
        "denom": "MCustomMarker"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
