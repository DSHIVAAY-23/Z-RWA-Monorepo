
$local_prov/provenanced tx wasm execute \
    tp1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqf06p2p \
    '{
    "manage_roles": {
        "denom": "CCustomMarker",
        "role": {
            "agent": {
                "update_type": {
                    "remove": [
                        "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy"
                    ]
                },
                "marker_access": [
                    "admin",
                    "burn",
                    "deposit",
                    "delete",
                    "mint",
                    "transfer",
                    "unspecified",
                    "withdraw",
                    "freeze",
                    "unfreeze",
                    "force_transfer"
                ]
            }
        }
    }
}
' \
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
