
provenanced query wasm contract-state smart tp15vaux4uwcehrkh5t6rhjctkr8z05tmpm9hc5hrgzeu0xxy2gvtxqhdlhyz \
	'{
  "get_balance_of": {
    "address": "tp1zu5rdmpk08epmlt4j6qejwgej203zz86thfns2"
  }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
  