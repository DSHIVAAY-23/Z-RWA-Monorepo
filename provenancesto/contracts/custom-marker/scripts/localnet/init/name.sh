$local_prov/provenanced tx name bind \
    "cm" \
    $feebucket \
    "pb" \
    --from $feebucket \
    --keyring-backend test \
    --home $local_prov_path \
    --chain-id testing \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	| jq
