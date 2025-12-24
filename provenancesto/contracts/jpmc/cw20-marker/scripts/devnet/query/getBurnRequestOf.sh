
provenanced query wasm contract-state smart tp1kzzd6jmc9d2844h9pz4mzwycy2qsuv2wsz2aq73uk3n924qqn6pqv30kc9 \
	'{
    "get_request_of": {
        "request_id": "0xb24420cb0602ca74be5d0e701ba32790f61c14ebb67ec4a64b839bc1ff821efb"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
