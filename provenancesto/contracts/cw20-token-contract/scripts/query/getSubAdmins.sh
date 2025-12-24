
provenanced query wasm contract-state smart tp19kwsg0vpaa20pf5xkzpyfkvthgm6vk6ztlrmcdhjnxrn620agzsqwqnqaz \
	'{
    "get_sub_admins": {}
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
