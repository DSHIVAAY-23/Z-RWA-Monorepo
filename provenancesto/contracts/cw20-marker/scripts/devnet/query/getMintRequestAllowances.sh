
provenanced query wasm contract-state smart tp1wkwy0xh89ksdgj9hr347dyd2dw7zesmtrue6kfzyml4vdtz6e5wsvaczas \
	'{
    "get_request_allowances": {
        "owner": "tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp",
        "spender": "tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp",
        "request_type": "mint"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
