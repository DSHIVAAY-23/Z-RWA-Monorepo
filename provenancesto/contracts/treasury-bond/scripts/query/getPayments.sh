
provenanced query wasm contract-state smart tp190s9ns0jvgr423jznaxcca332kze964dqpf64rr47vxrx2lag04sney6sx \
	'{
    "get_payments": {
        "denom": "MCustomMarker",
        "user": "tp17wka6zmfq2q0vqeg2zhrmj46327v5npuhf4dq7"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq