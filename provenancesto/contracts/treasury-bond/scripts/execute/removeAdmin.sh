provenanced tx wasm execute \
    tp190s9ns0jvgr423jznaxcca332kze964dqpf64rr47vxrx2lag04sney6sx \
    '{
    "manage_roles": {
        "role": {
            "admin": {
                "update_type": {
                    "remove": [
                        "tp17wka6zmfq2q0vqeg2zhrmj46327v5npuhf4dq7"
                    ]
                }
            }
        }
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
