
provenanced tx wasm execute \
    tp1kzzd6jmc9d2844h9pz4mzwycy2qsuv2wsz2aq73uk3n924qqn6pqv30kc9 \
    '{
    "manage_roles": {
        "roles": [
            {
                "tokenization_agent": {
                    "update_type": {
                        "add": "tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp"
                    }
                }
            }
        ]
    }
}' \
    --from $dev \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-axl-devnet-1 \
    --gas auto \
    --gas-prices 0vspn \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq

