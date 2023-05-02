#!/bin/sh

rm -rf $HOME/.ununifi/

cd $HOME

chain_id=test

echo "figure web rescue rice quantum sustain alert citizen woman cable wasp eyebrow monster teach hockey giant monitor hero oblige picnic ball never lamp distance" > mnt.txt;

ununifid init --chain-id=$chain_id ununifi-test --home=$HOME/.ununifi
ununifid keys add validator --keyring-backend=test --home=$HOME/.ununifi
ununifid keys add user1 --recover --keyring-backend=test < mnt.txt;
ununifid genesis add-genesis-account $(ununifid keys show validator -a --keyring-backend=test --home=$HOME/.ununifi) 100000000000stake --home=$HOME/.ununifi
ununifid genesis add-genesis-account $(ununifid keys show user1 -a --keyring-backend=test --home=$HOME/.ununifi) 10000000stake --home=$HOME/.ununifi
ununifid genesis gentx validator 500000000stake --keyring-backend=test --home=$HOME/.ununifi --chain-id=$chain_id
ununifid genesis collect-gentxs --home=$HOME/.ununifi

ununifid start --home=$HOME/.ununifi