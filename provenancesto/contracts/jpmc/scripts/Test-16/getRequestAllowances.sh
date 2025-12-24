
provenanced query wasm contract-state smart tp1lu8mr2ge4hz7kd5qky2ysayyrtrgst7yslqd5ekuuqp79m9g9k0qwdeq54 \
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
