provenanced tx wasm migrate \
    tp15vaux4uwcehrkh5t6rhjctkr8z05tmpm9hc5hrgzeu0xxy2gvtxqhdlhyz \
    90 \
    '{}' \
    --from "$dev" \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-axl-devnet-1 \
    --broadcast-mode block \
    --testnet \
    --yes \
    --gas auto \
    --gas-prices 0vspn \
    --node=http://34.70.126.95:26657 \
	--output json | jq

