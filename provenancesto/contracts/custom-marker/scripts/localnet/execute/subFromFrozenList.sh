
$local_prov/provenanced tx wasm execute \
    tp1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqf06p2p \
    '{
    "update_frozen_list": {
        "sub": {
            "address": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
            "amount": "100"
        }
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
