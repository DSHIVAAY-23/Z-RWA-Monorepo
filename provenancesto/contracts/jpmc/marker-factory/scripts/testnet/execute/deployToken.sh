
provenanced tx wasm execute \
    tp1ve5ydzcdrpww2s6mdncw8qvqaemgp9cp9zacn7jcter9hz8anscqf2t070 \
    '{
    "deploy_token": {
        "msg": {
            "denom": "TJTest-4",
            "tokenization_agent": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
            "config": {
                "chain": "provenance",
                "address": "tp1wjea0da3kzt7rcddjyvf9gxf7nkvzuc89dkfrdh3ywqnz7kt6z5qxpkvxk"
            }
        }
    }
}' \
    --from $feebucket \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas-prices 400000nhash \
    --gas auto \
    --gas-adjustment 1.3 \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq

