
provenanced query wasm contract-state smart tp1v67pppdudcpdddkn8wlgwh7fzwrqjqw7juwxe63fmtnwf758s5fsa7qrla \
	'{
  "get_denom_by_contract": {
    "addr": "tp1ysjq08nlz4jh06spcfmv3fyewc2lrx80ldtlzw66rs4pnmnygl0svjmt5m"
  }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
