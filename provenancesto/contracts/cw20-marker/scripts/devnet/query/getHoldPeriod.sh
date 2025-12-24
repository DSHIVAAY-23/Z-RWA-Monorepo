
provenanced query wasm contract-state smart tp1wkwy0xh89ksdgj9hr347dyd2dw7zesmtrue6kfzyml4vdtz6e5wsvaczas \
	'{
    "get_hold_period": {
        "denom": "OasisToken"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
