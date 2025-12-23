
echo balance for $tarun
provenanced query wasm contract-state smart tp1h7scjsemhcy9pgs9ges488n5cxdghz4sj2qk0qwgrjw7jrpgvets8j4tzq \
	'{
  "get_balance_of": {
    "address": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy"
  }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq

echo balance for $minter
provenanced query wasm contract-state smart tp1h7scjsemhcy9pgs9ges488n5cxdghz4sj2qk0qwgrjw7jrpgvets8j4tzq \
	'{
  "get_balance_of": {
    "address": "tp1c7apkgl4l8pw72nsh6uvvr7d6fy3c98hyg0sct"
  }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq