
provenanced query wasm contract-state smart tp19kwsg0vpaa20pf5xkzpyfkvthgm6vk6ztlrmcdhjnxrn620agzsqwqnqaz \
	'{
  "allowance": {
    "owner": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
    "spender": "tp1m97r57ms8dl7pxn0j4m7w80d6a5qvdp7pns80g"
  }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
