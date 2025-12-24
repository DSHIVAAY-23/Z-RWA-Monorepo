

provenanced tx wasm execute \
    tp1v67pppdudcpdddkn8wlgwh7fzwrqjqw7juwxe63fmtnwf758s5fsa7qrla \
    '{
    "deploy_token": {
        "msg": {
            "denom": "APO8",
            "tokenization_agent": "tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp",
            "config": {
                "chain": "onyx",
                "address": "0x466Ca44aC65567F87e3D1E69e0D16E2716a37B58"
            }
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

