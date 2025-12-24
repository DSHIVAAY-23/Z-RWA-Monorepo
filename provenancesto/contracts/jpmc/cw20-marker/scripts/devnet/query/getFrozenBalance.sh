
provenanced query wasm contract-state smart tp1kzzd6jmc9d2844h9pz4mzwycy2qsuv2wsz2aq73uk3n924qqn6pqv30kc9 \
	'{
  "get_frozen_balance": {
    "address": "tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp"
  }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq