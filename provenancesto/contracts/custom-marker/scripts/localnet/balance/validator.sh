$local_prov/provenanced --testnet \
   --keyring-dir "build/run/$local_prov/provenanced/" \
   --chain-id tendermint \
   --home "$PIO_HOME" \
tx staking create-validator \
   --moniker arbiter34.com \
   --pubkey "tpvalconspub1zcjduepqqwlutz7sqt7rhs4wveu4dlfj2u8323jrc7hu0f4xdgpss8tug7esnqewhv" \
   --amount 9000000000nhash \
   --from stakeholder0 \
   --fees 5000nhash \
   --commission-rate=1.0 \
   --commission-max-rate=1.0 \
   --commission-max-change-rate=1.0 \
   --min-self-delegation 1 \
   --broadcast-mode block
