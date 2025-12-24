$local_prov/provenanced query wasm contract-state smart tp1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqf06p2p \
	'{
    "get_by_address": {
        "address": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy"
    }
}' \
    --testnet \
	--output json \
	| jq
