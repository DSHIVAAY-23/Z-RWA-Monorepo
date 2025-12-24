
$local_prov/provenanced tx marker grant-authz \
    tp1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqf06p2p \
    transfer \
    --transfer-limit=1000CustomMarker \
    --from "$tarun" \
    --keyring-backend test \
    --home $local_prov_path \
    --chain-id testing \
    --broadcast-mode block \
    --testnet \
    --yes \
    --gas 4000000 \
    --gas-prices 1905nhash \
    --node=https://rpc.test.provenance.io:443 \
	--output json | jq
