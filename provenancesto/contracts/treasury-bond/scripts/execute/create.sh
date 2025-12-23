provenanced tx wasm execute \
    tp190s9ns0jvgr423jznaxcca332kze964dqpf64rr47vxrx2lag04sney6sx \
    '{
    "create": {
        "params": {
            "denom": "CustomMarker",
            "issue_size": "1",
            "face_value": "100",
            "coupon_rate": 8,
            "accrued_interest": 7,
            "maturity_date": 1,
            "issuer_name": "issuer",
            "coupon_frequency": "Monthly"
        }
    }
}' \
    --from $feebucket \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 26905nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq '.txhash, .code, .raw_log'
