
provenanced tx wasm execute \
    tp1wjea0da3kzt7rcddjyvf9gxf7nkvzuc89dkfrdh3ywqnz7kt6z5qxpkvxk \
    '{
    "send_message_evm": {
        "destination_chain": "Polygon",
        "destination_address": "0x7f476D898B3b6990e2de802A0B7aC73756E8B08b",
        "message": "Hello viswa message 3!",
        "msg_type": "message"
    }
}' \
    --from $validator \
    --amount 5000000ibc/F53E48CE45EF24BD633402397B0013E02013E9ABD420FEAB905EAF01B8F15DD0 \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 26905nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq '.txhash, .code, .raw_log'

