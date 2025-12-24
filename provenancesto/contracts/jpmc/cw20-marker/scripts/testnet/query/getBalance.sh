
provenanced query wasm contract-state smart tp1h7scjsemhcy9pgs9ges488n5cxdghz4sj2qk0qwgrjw7jrpgvets8j4tzq \
	'{
  "get_balance": {
    "address": "tp1gpw2r2ga427d6trrsxq8l8axjgmmh8vwxda4gm",
    "denom": "USDT-Test"
  }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq



provenanced query wasm contract-state smart tp1h7scjsemhcy9pgs9ges488n5cxdghz4sj2qk0qwgrjw7jrpgvets8j4tzq \
	'{
  "get_balance": {
    "address": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
    "denom": "MCustomMarker"
  }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
