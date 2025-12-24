
provenanced tx wasm execute \
    tp1gdrv7296qj3f5yuxvtrqc43dp6nc4dwdw4f9hhpwe9n759jd0lushvxcu8 \
    '{
    "call": {
        "to": "tp1ujwgh8hs0l235plvln6qd0jxgxjym92z22chn7",
        "amount": "5"
    }
}' \
    --from $validator \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 26905nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq

