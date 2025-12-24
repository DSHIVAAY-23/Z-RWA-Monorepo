#provenanced q bank balances $1 -t -o json | jq

echo $tarun
$local_prov/provenanced --testnet query bank balances $tarun 

echo $minter
$local_prov/provenanced --testnet query bank balances $minter 

echo $feebucket
$local_prov/provenanced --testnet query bank balances $feebucket 