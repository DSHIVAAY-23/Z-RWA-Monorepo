
provenanced tx wasm execute \
    tp1chaxg0l0vy4j48x3hr77663u6e5kx3akc74nadra0mqjfjks9r9sqnzrzx \
    '{
    "manage_roles": {
        "roles": [
            {
                "validators": {
                    "update_type": {
                        "add": [
                            "tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp",
                            "tp1zu5rdmpk08epmlt4j6qejwgej203zz86thfns2"
                        ]
                    }
                }
            }
        ]
    }
}' \
    --from $feebucket \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 26905nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
