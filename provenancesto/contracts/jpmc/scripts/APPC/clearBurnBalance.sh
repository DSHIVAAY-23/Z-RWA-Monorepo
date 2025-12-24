
provenanced tx wasm execute \
    tp1jlefmscn0yfyg5fm2vwjxazwxxgtnp20dvx2fauvm7af2ucevvdqd3afls \
    '{
    "clear_burn_balance": {
        "address": "tp1zu5rdmpk08epmlt4j6qejwgej203zz86thfns2"
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
