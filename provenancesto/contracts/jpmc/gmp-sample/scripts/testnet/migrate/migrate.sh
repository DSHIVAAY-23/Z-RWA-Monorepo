provenanced tx wasm migrate \
    tp1gdrv7296qj3f5yuxvtrqc43dp6nc4dwdw4f9hhpwe9n759jd0lushvxcu8 \
    577 \
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
	--output json | jq

