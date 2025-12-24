
provenanced query wasm contract-state smart tp1v67pppdudcpdddkn8wlgwh7fzwrqjqw7juwxe63fmtnwf758s5fsa7qrla \
	'{
    "get_freezed_accounts": {
		"denom": "Test-1"
	}
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
