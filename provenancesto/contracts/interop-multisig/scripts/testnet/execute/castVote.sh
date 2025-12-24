
provenanced tx wasm execute \
    tp1chaxg0l0vy4j48x3hr77663u6e5kx3akc74nadra0mqjfjks9r9sqnzrzx \
    '{
    "cast_vote": {
        "tx_hash" : "9EA87D6B8F3269F95D763A6C05AF63B26998FBEB5D2D8F2CCCFF362CD8FEEEFD",
        "can_transact": true
    }
}' \
    --from $feebucket \
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
