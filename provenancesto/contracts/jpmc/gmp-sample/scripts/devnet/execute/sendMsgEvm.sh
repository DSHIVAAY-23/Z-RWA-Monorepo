
provenanced tx wasm execute \
    tp14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s96lrg8 \
    '{
    "send_message_evm": {
        "destination_chain": "onyx",
        "destination_address": "0xD021c77149466aa2640878cb34D5Ab5e4Be3326d",
        "message": "unpartial_freeze#OasisToken#tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp#100",
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

