
provenanced query wasm contract-state smart tp1ve5ydzcdrpww2s6mdncw8qvqaemgp9cp9zacn7jcter9hz8anscqf2t070 \
	'{
  "get_balance": {
    "denom": "WCustomMarker",
    "address": "tp1gpw2r2ga427d6trrsxq8l8axjgmmh8vwxda4gm",
    "denom": "USDT-Test"
  }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq



provenanced query wasm contract-state smart tp1ve5ydzcdrpww2s6mdncw8qvqaemgp9cp9zacn7jcter9hz8anscqf2t070 \
	'{
  "get_balance": {
    "address": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
    "denom": "MCustomMarker"
  }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
