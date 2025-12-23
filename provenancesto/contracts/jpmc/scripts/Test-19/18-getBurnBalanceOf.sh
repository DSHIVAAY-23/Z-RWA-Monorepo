
provenanced query wasm contract-state smart tp16t96gk6telazd5f9uvxynxdtzf0503vqw6fsvrkq2lxzg9vmc98qdyavdg \
	'{
    "get_burn_balance_of": {
        "owner": "tp1zu5rdmpk08epmlt4j6qejwgej203zz86thfns2"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
