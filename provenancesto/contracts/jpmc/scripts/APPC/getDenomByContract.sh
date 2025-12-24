
provenanced query wasm contract-state smart tp1v67pppdudcpdddkn8wlgwh7fzwrqjqw7juwxe63fmtnwf758s5fsa7qrla \
	'{
  "get_denom_by_contract": {
    "addr": "tp1jlefmscn0yfyg5fm2vwjxazwxxgtnp20dvx2fauvm7af2ucevvdqd3afls"
  }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
