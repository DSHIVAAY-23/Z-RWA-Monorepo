
provenanced query wasm contract-state smart tp10m7er24gc7u0fl26qpm4d487d90vug2gw2s4kq9r5zw00nd4hymqgmrpa3 \
	'{
  "get_frozen_balance": {
    "address": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
    "denom": "MCustomMarker"
  }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq