provenanced tx wasm migrate \
    tp16h50hcp3m777t68vv42x6kzdrym9dyn5ucxq6tpj46qnnye0k97slzkku3 \
    686 \
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
