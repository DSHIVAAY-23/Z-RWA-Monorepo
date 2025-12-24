
provenanced query wasm contract-state smart tp1v67pppdudcpdddkn8wlgwh7fzwrqjqw7juwxe63fmtnwf758s5fsa7qrla \
	'{
  "get_denom_by_contract": {
    "addr": "tp1m37cxl0ld4uaw3r4lv9nt2uw69xxf8xfjrf7a4w9hamv6xvp6ddq528hvs"
  }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
