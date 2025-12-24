
provenanced tx wasm execute \
    tp16t96gk6telazd5f9uvxynxdtzf0503vqw6fsvrkq2lxzg9vmc98qdyavdg \
    '{
    "update_dest_config": {
        "config": {
            "chain": "onyx",
            "address": "0xebf2bb22e15B29514C62034eDdcc372Bf6CddF31"
        }
    }
}' \
    --from $dev \
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

