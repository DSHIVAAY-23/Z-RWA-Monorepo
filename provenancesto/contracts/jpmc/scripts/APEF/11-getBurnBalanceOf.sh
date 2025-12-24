
provenanced query wasm contract-state smart tp1emy7mkz3p3kcq3vmxjacud2m255gutwykplyns7z6yemp6mnqppsvlfnup \
	'{
    "get_burn_balance_of": {
        "owner": "tp1zu5rdmpk08epmlt4j6qejwgej203zz86thfns2"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
