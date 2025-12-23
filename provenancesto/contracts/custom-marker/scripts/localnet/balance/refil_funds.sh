$local_prov/provenanced tx bank send \
	"$local_validator" \
	"$tarun" \
	200000000000nhash \
	--from="$validator" \
	--keyring-backend=test \
	--home=$local_prov_path \
	--chain-id=testing \
	--gas=auto \
	--gas-prices="1905nhash" \
	--gas-adjustment=1.5 \
	--broadcast-mode=block \
	--yes \
	--testnet \
	--output json | jq

$local_prov/provenanced tx bank send \
	"$local_validator" \
	"$feebucket" \
	200000000000nhash \
	--from="$validator" \
	--keyring-backend=test \
	--home=$local_prov_path \
	--chain-id=testing \
	--gas=auto \
	--gas-prices="1905nhash" \
	--gas-adjustment=1.5 \
	--broadcast-mode=block \
	--yes \
	--testnet \
	--output json | jq

$local_prov/provenanced tx bank send \
	"$local_validator" \
	"$local_validator" \
	200000000000nhash \
	--from="$local_validator" \
	--keyring-backend=test \
	--home=$local_prov_path \
	--chain-id=testing \
	--gas=auto \
	--gas-prices="1905nhash" \
	--gas-adjustment=1.5 \
	--broadcast-mode=block \
	--yes \
	--testnet \
	--output json | jq

$local_prov/provenanced tx bank send \
	"$local_validator" \
	"$local_issuer" \
	200000000000nhash \
	--from="$validator" \
	--keyring-backend=test \
	--home=$local_prov_path \
	--chain-id=testing \
	--gas=auto \
	--gas-prices="1905nhash" \
	--gas-adjustment=1.5 \
	--broadcast-mode=block \
	--yes \
	--testnet \
	--output json | jq

$local_prov/provenanced tx bank send \
	"$local_validator" \
	"$local_tokenization_agent" \
	200000000000nhash \
	--from="$validator" \
	--keyring-backend=test \
	--home=$local_prov_path \
	--chain-id=testing \
	--gas=auto \
	--gas-prices="1905nhash" \
	--gas-adjustment=1.5 \
	--broadcast-mode=block \
	--yes \
	--testnet \
	--output json | jq

$local_prov/provenanced tx bank send \
	"$local_validator" \
	"$local_transfer_agent" \
	200000000000nhash \
	--from="$validator" \
	--keyring-backend=test \
	--home=$local_prov_path \
	--chain-id=testing \
	--gas=auto \
	--gas-prices="1905nhash" \
	--gas-adjustment=1.5 \
	--broadcast-mode=block \
	--yes \
	--testnet \
	--output json | jq
