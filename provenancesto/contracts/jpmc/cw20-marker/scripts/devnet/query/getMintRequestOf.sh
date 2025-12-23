
provenanced query wasm contract-state smart tp1kzzd6jmc9d2844h9pz4mzwycy2qsuv2wsz2aq73uk3n924qqn6pqv30kc9 \
	'{
    "get_request_of": {
        "request_id": "0x46ad308406ad6268f6d6e2aeecc81eb321ff3ef1ff1730cfe96a203edb1d6f39"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
