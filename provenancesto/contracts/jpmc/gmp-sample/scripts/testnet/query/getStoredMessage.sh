
provenanced query wasm contract-state smart tp1gdrv7296qj3f5yuxvtrqc43dp6nc4dwdw4f9hhpwe9n759jd0lushvxcu8 \
	'{
    "get_stored_message": {}
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
