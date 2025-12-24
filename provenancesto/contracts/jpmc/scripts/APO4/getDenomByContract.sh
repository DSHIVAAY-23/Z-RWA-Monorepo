
provenanced query wasm contract-state smart tp1v67pppdudcpdddkn8wlgwh7fzwrqjqw7juwxe63fmtnwf758s5fsa7qrla \
	'{
  "get_denom_by_contract": {
    "addr": "tp1ucrz2ntndpxhdnz2hr55gqwjp3jpmpfrjucd2tvka60dq7rujklq0j0yq5"
  }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
