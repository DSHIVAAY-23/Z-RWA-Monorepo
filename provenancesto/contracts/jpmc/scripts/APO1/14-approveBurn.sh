
provenanced tx wasm execute \
    tp1ysjq08nlz4jh06spcfmv3fyewc2lrx80ldtlzw66rs4pnmnygl0svjmt5m \
    '{
    "approve_request": {
        "request_id": "0x000000000000000000000007cc3b6d7532b7d5eebd9b6ff435089b5e4e703b59",
        "request_type": "burn"
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
	--node=http://34.70.126.95:26657 |  jq '.txhash, .raw_log'
