read -p "Loop until: " n
for i in $(seq 1 $n); do
    curl 'https://test.provenance.io/blockchain/faucet/external'  \
  -X 'POST'  -H 'Content-Type: application/json'  \
--data-binary '{"address":"tp1jlv0ytdm2klehl2w3z9a45q9vksu9tnkv5wx88"}'
done

