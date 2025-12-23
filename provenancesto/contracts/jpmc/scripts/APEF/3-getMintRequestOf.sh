
provenanced query wasm contract-state smart tp1emy7mkz3p3kcq3vmxjacud2m255gutwykplyns7z6yemp6mnqppsvlfnup \
	'{
    "get_request_of": {
        "request_id": "0x000000000000000000000007a5b63558c8bbc8cccf2aea7a764eb8b4857c1bfa"
    }
}' \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
