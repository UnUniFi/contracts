#!/bin/bash

set -o errexit -o nounset

CONTRACT_ADDR=osmo14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sq2r9g9
NODE=http://localhost:26653
CHAIN_ID=osmo-testing
OSMO_HOME=$HOME/.osmosisd/validator1
TXFLAG="--gas=auto --gas-prices 0.1uosmo --gas-adjustment 1.3 -b block --keyring-backend=test --home=$OSMO_HOME --node $NODE --chain-id $CHAIN_ID"
QFLAG="--home $OSMO_HOME --node $NODE"

EXECUTE_JOIN_SWAP_MSG='{"join_swap_extern" : {"pool_id":1, "token_in": {"denom": "uosmo", "amount":"10"}, "share_out_min_amount":"1"}}'
osmosisd tx wasm execute $CONTRACT_ADDR "$EXECUTE_JOIN_SWAP_MSG" --from validator1 $TXFLAG -y \
    --amount 10uosmo 

# query for the share amount of the depositor
DEPOSITOR=$(osmosisd keys show -a validator1 --keyring-backend=test --home $HOME/.osmosisd/validator1)
EXECUTE_DEPOSITOR_SHARE_QUERY='{"depositor_share_amount": {"depositor": "'$DEPOSITOR'"}}'
osmosisd q wasm contract-state smart $CONTRACT_ADDR "$EXECUTE_DEPOSITOR_SHARE_QUERY" $QFLAG

# query for the pool assets
osmosisd q gamm pool 1 $QFLAG

osmosisd q poolincentives gauge-ids 1  $QFLAG

osmosisd q incentives gauge-by-id 1 $QFLAG

osmosisd q incentives to-distribute-coins $QFLAG