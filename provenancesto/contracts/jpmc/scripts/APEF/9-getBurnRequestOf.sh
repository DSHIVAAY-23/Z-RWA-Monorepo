
provenanced query wasm contract-state smart tp1emy7mkz3p3kcq3vmxjacud2m255gutwykplyns7z6yemp6mnqppsvlfnup \
	'{
    "get_request_of": {
        "request_id": "0x00000000000000000000000712c97c8069b288cf1427cd918099ee153c3e0c11"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
