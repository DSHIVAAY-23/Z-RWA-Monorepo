
provenanced tx wasm execute \
    tp1chaxg0l0vy4j48x3hr77663u6e5kx3akc74nadra0mqjfjks9r9sqnzrzx \
    '{
    "manage_roles": {
        "roles": [
            {
                "admins": {
                    "update_type": {
                        "remove": [
                            "tp1esdv3xcal3fmc5aq93wvsujgkvc75mnf09jc73"
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
