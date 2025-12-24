
provenanced query wasm contract-state smart tp16h50hcp3m777t68vv42x6kzdrym9dyn5ucxq6tpj46qnnye0k97slzkku3 \
	'{
  "get_frozen_balance": {
    "address": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
    "denom": "MCustomMarker"
  }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq