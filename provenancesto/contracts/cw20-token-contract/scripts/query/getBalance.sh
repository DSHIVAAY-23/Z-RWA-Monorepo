echo "Balance for account: tp1f8jfpg8tmewcsu2jjwwkhnzun6d30fzjrghvqs"
provenanced query wasm contract-state smart tp19kwsg0vpaa20pf5xkzpyfkvthgm6vk6ztlrmcdhjnxrn620agzsqwqnqaz \
	'{
    "balance": {
        "address": "tp1f8jfpg8tmewcsu2jjwwkhnzun6d30fzjrghvqs"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq

echo
echo "Balance for account: $minter"
provenanced query wasm contract-state smart tp19kwsg0vpaa20pf5xkzpyfkvthgm6vk6ztlrmcdhjnxrn620agzsqwqnqaz \
	'{
    "balance": {
        "address": "tp1c7apkgl4l8pw72nsh6uvvr7d6fy3c98hyg0sct"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq

echo
echo "Balance for account: $validator"
provenanced query wasm contract-state smart tp19kwsg0vpaa20pf5xkzpyfkvthgm6vk6ztlrmcdhjnxrn620agzsqwqnqaz \
	'{
    "balance": {
        "address": "tp1m97r57ms8dl7pxn0j4m7w80d6a5qvdp7pns80g"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq

echo
echo "Balance for account: $tarun"
provenanced query wasm contract-state smart tp19kwsg0vpaa20pf5xkzpyfkvthgm6vk6ztlrmcdhjnxrn620agzsqwqnqaz \
	'{
    "balance": {
        "address": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq