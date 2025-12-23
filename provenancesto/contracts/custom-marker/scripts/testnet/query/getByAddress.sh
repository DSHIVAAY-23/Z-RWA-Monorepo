provenanced query wasm contract-state smart tp16h50hcp3m777t68vv42x6kzdrym9dyn5ucxq6tpj46qnnye0k97slzkku3 \
	'{
    "get_by_address": {
        "address": "tp16w76c438q7u5qxuzmamc82m2end42ykf0y3dca"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
