
provenanced tx wasm execute \
    tp1ftwkz60sshd3pzmrrkzgfp764enkd8qk8fdntfkj8rhef3rg0zuqwf7f3k \
    '{
    "send_message_cosmos": {
        "destination_chain": "Avalanche",
        "destination_address": "0x07EE2b63937945De2afA918A5aeDaa543e51be40",
        "message": "hello!",
        "msg_type": "message"
    }
}' \
    --from $validator \
    --amount 1nhash \
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

