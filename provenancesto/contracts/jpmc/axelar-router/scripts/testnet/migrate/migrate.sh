provenanced tx wasm migrate \
    tp1ftwkz60sshd3pzmrrkzgfp764enkd8qk8fdntfkj8rhef3rg0zuqwf7f3k \
    546 \
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

