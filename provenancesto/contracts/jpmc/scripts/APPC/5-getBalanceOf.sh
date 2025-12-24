
provenanced query wasm contract-state smart tp1jlefmscn0yfyg5fm2vwjxazwxxgtnp20dvx2fauvm7af2ucevvdqd3afls \
	'{
  "get_balance_of": {
    "address": "tp1zu5rdmpk08epmlt4j6qejwgej203zz86thfns2"
  }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
  