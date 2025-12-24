
provenanced tx wasm execute \
    tp1d8lzewx67da62k4ax5gcz4h90w236gnehfhx65y5ly24zwgdcyuscc48wx \
    '{
    "send_message_evm": {
        "destination_chain": "onyx",
        "destination_address": "0xD021c77149466aa2640878cb34D5Ab5e4Be3326d",
        "message": "Hello Ganesh and Abhishek!",
        "msg_type": "message"
    }
}' \
    --from $dev \
    --amount 1vspn \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-axl-devnet-1 \
    --gas 4000000 \
    --gas-prices 0vspn \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq

