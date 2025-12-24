provenanced tx wasm migrate \
    tp190s9ns0jvgr423jznaxcca332kze964dqpf64rr47vxrx2lag04sney6sx \
    663 \
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