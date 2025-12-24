
provenanced query wasm contract-state smart tp1v67pppdudcpdddkn8wlgwh7fzwrqjqw7juwxe63fmtnwf758s5fsa7qrla \
	'{
    "get_request_allowances": {
        "denom": "Test-1",
        "owner": "tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp",
        "spender": "tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp",
        "request_type": "mint"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
