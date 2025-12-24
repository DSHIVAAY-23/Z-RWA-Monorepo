
provenanced query wasm contract-state smart tp1v67pppdudcpdddkn8wlgwh7fzwrqjqw7juwxe63fmtnwf758s5fsa7qrla \
	'{
  "get_denom_by_contract": {
    "addr": "tp1xgzc3t3x3jshwfst83umkxthckh0dyqexa54dsuyyh00jsgs7tlse0a9ty"
  }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
