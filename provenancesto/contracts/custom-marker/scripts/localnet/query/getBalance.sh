
$local_prov/provenanced query wasm contract-state smart tp1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqf06p2p \
	'{
  "get_balance": {
    "address": "tp1y49xa3cl0qsa7lr0vhasgsp6mnpcun8qc3n7nq",
    "denom": "CCustomMarker"
  }
}' \
    --testnet \
	--output json \
	| jq