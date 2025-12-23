$local_prov/provenanced \
	-t \
	--home $local_prov_path \
	 keys \
	 list \
	 --keyring-backend test

$local_prov/provenanced -t --home $local_prov_path keys list --keyring-backend test