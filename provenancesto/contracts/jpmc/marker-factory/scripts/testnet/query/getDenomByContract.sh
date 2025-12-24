
provenanced query wasm contract-state smart tp1ve5ydzcdrpww2s6mdncw8qvqaemgp9cp9zacn7jcter9hz8anscqf2t070 \
	'{
  "get_denom_by_contract": {
    "addr": "tp1zu078p94v4hfy32nu70z264wah52ypzpxhz38kqqhk8cz3hd3c4q6ca248"
  }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
