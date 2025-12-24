
provenanced query wasm contract-state smart tp1we9chrq58nj3etdw3yss7ytqeaeeka5725u2m0lg7vg5yzrwa8usp8shgj \
	'{
    "get_request_allowances": {
        "owner": "tp1zu5rdmpk08epmlt4j6qejwgej203zz86thfns2",
        "spender": "tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp",
        "request_type": "burn"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
