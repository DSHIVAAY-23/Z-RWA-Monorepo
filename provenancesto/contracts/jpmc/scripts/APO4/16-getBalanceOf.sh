
provenanced query wasm contract-state smart tp1ucrz2ntndpxhdnz2hr55gqwjp3jpmpfrjucd2tvka60dq7rujklq0j0yq5 \
	'{
  "get_balance_of": {
    "address": "tp1zu5rdmpk08epmlt4j6qejwgej203zz86thfns2"
  }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
  