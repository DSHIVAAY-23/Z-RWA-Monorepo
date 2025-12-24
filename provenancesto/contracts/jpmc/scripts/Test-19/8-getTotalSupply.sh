
provenanced query wasm contract-state smart tp16t96gk6telazd5f9uvxynxdtzf0503vqw6fsvrkq2lxzg9vmc98qdyavdg \
	'{
    "get_total_supply": {}
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
