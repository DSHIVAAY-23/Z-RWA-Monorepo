
provenanced tx wasm execute \
    tp1d8lzewx67da62k4ax5gcz4h90w236gnehfhx65y5ly24zwgdcyuscc48wx \
    '{
    "send_message_cosmos": {
        "destination_chain": "provenance",
        "destination_address": "tp1d8lzewx67da62k4ax5gcz4h90w236gnehfhx65y5ly24zwgdcyuscc48wx",
        "message": "requestMintFrom|Test-16|0x1223311|tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp|100",
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
