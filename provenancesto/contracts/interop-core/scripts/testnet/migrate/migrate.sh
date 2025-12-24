provenanced tx wasm migrate \
    tp1hfcpqqxl0e9g6terx5qw0nvqrfty9thequ6c8czc9k7vytyd98ys9pj40a \
    659 \
    '{}' \
    --from "$feebucket" \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --broadcast-mode block \
    --testnet \
    --yes \
    --gas 4000000 \
    --gas-prices 26905nhash \
    --node=https://rpc.test.provenance.io:443 \
	--output json | jq '.txhash, .code, .raw_log'

