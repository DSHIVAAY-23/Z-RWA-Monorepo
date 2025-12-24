
provenanced query wasm contract-state smart tp1h7scjsemhcy9pgs9ges488n5cxdghz4sj2qk0qwgrjw7jrpgvets8j4tzq \
	'{
  "get_allowance": {
    "owner": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
    "spender": "tp1gpw2r2ga427d6trrsxq8l8axjgmmh8vwxda4gm"
  }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
