
provenanced query wasm contract-state smart tp1v67pppdudcpdddkn8wlgwh7fzwrqjqw7juwxe63fmtnwf758s5fsa7qrla \
	'{
  "get_denom_by_contract": {
    "addr": "tp1qfqq9tz0s7f57dwmd7zmxwhwvjcqvcwv5y7uvrr7xvkwc9uy68wqch75zx"
  }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
