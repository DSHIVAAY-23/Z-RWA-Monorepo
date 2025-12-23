APEF=tp1emy7mkz3p3kcq3vmxjacud2m255gutwykplyns7z6yemp6mnqppsvlfnup
APO1=tp1ysjq08nlz4jh06spcfmv3fyewc2lrx80ldtlzw66rs4pnmnygl0svjmt5m
APO4=tp1ucrz2ntndpxhdnz2hr55gqwjp3jpmpfrjucd2tvka60dq7rujklq0j0yq5
APO8=tp17xethxxpfq6rtg6wwzupkfmrc8mrt0k2s4c6nw30v90anlhx9mrqjndeec
APO9=tp1qfqq9tz0s7f57dwmd7zmxwhwvjcqvcwv5y7uvrr7xvkwc9uy68wqch75zx
APPC=tp1jlefmscn0yfyg5fm2vwjxazwxxgtnp20dvx2fauvm7af2ucevvdqd3afls
APPE=tp1xgzc3t3x3jshwfst83umkxthckh0dyqexa54dsuyyh00jsgs7tlse0a9ty
APRE=tp15vaux4uwcehrkh5t6rhjctkr8z05tmpm9hc5hrgzeu0xxy2gvtxqhdlhyz

# ./increaseBurnRequestAllowance.sh $APO9 $holding101
# ./increaseBurnRequestAllowance.sh $APO9 $holding102
# ./increaseBurnRequestAllowance.sh $APO9 $holding103

# ./increaseMintRequestAllowance.sh $APO9 $holding101
# ./increaseMintRequestAllowance.sh $APO9 $holding102
# ./increaseMintRequestAllowance.sh $APO9 $holding103

contracts=($APEF $APO1 $APO4 $APO8 $APO9 $APPC $APPE $APRE)
accounts=($user $holding101 $holding102 $holding103)
# echo "${contracts[@]}"

for contract in "${contracts[@]}"
do
    for account in "${accounts[@]}"
    do
        echo "$contract" "$account"
    done
done
