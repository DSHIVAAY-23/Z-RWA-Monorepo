
provenanced tx wasm execute \
    tp1wjea0da3kzt7rcddjyvf9gxf7nkvzuc89dkfrdh3ywqnz7kt6z5qxpkvxk \
    '{
    "send_message_cosmos": {
        "destination_chain": "provenance",
        "destination_address": "tp1wjea0da3kzt7rcddjyvf9gxf7nkvzuc89dkfrdh3ywqnz7kt6z5qxpkvxk",
        "message": "mint|4|TJTest-3|tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy|100",
        "msg_type": "message"
    }
}' \
    --from $validator \
    --amount 1000000ibc/F53E48CE45EF24BD633402397B0013E02013E9ABD420FEAB905EAF01B8F15DD0 \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 26905nhash \
    --broadcast-mode block \
    --testnet \
    --yes \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq '.txhash, .code, .raw_log'
