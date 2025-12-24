
provenanced tx wasm execute \
    tp1kzzd6jmc9d2844h9pz4mzwycy2qsuv2wsz2aq73uk3n924qqn6pqv30kc9 \
    '{
    "send_message_cosmos": {
        "destination_chain": "provenance",
        "destination_address": "tp1d8lzewx67da62k4ax5gcz4h90w236gnehfhx65y5ly24zwgdcyuscc48wx",
        "message": "Response Reached?",
        "msg_type": "message"
    }
}' \
    --from $dev \
    --amount 1vspn \
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
