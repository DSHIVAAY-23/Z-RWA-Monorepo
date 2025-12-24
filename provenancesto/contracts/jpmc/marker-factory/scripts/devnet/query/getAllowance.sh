
provenanced query wasm contract-state smart tp1v67pppdudcpdddkn8wlgwh7fzwrqjqw7juwxe63fmtnwf758s5fsa7qrla \
	'{
    "get_allowance": {
        "denom": "APO1",
        "owner": "tp1ysjq08nlz4jh06spcfmv3fyewc2lrx80ldtlzw66rs4pnmnygl0svjmt5m",
        "spender": "tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq   