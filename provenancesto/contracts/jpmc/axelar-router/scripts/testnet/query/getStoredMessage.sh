
provenanced query wasm contract-state smart tp1ftwkz60sshd3pzmrrkzgfp764enkd8qk8fdntfkj8rhef3rg0zuqwf7f3k \
	'{
    "get_stored_message": {}
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
