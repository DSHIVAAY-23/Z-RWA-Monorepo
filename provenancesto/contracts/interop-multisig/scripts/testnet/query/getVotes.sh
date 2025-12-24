
provenanced query wasm contract-state smart tp1chaxg0l0vy4j48x3hr77663u6e5kx3akc74nadra0mqjfjks9r9sqnzrzx \
	'{
    "get_votes": {
        "tx_hash" : "0xc4d7b58b49c2f9e8aa46a0cf217cfbfd86891917258caacd05292659a3793305"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
