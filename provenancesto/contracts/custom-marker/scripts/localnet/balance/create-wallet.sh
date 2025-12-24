$local_prov/provenanced keys add local_tarun \
    --home $local_prov_path \
    --keyring-backend test \
    --testnet \
    --hd-path "44'/1'/0'/0/0" \
    --output json \
    | jq

$local_prov/provenanced keys add local_minter \
    --home $local_prov_path \
    --keyring-backend test \
    --testnet \
    --hd-path "44'/1'/0'/0/0" \
    --output json \
    | jq

$local_prov/provenanced keys add local_issuer \
    --home $local_prov_path \
    --keyring-backend test \
    --testnet \
    --hd-path "44'/1'/0'/0/0" \
    --output json \
    | jq

$local_prov/provenanced keys add local_tokenization_agent \
    --home $local_prov_path \
    --keyring-backend test \
    --testnet \
    --hd-path "44'/1'/0'/0/0" \
    --output json \
    | jq

    $local_prov/provenanced keys add local_transfer_agent \
    --home $local_prov_path \
    --keyring-backend test \
    --testnet \
    --hd-path "44'/1'/0'/0/0" \
    --output json \
    | jq
