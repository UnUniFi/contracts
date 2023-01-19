#!/bin/bash

set -o errexit -o nounset -u

echo "msg and quereis"

CONTRACT_ADDR=osmo14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sq2r9g9
NODE=http://localhost:26653
CHAIN_ID=osmo-testing
OSMO_HOME=$HOME/.osmosisd/validator1
TXFLAG="--gas=auto --gas-prices 0.1uosmo --gas-adjustment 1.3 -b block --keyring-backend=test --home=$OSMO_HOME --node $NODE --chain-id $CHAIN_ID"
QFLAG="--home $OSMO_HOME --node $NODE -o json"
DEPOSITOR=$(osmosisd keys show -a validator1 --keyring-backend=test --home $OSMO_HOME)

BALANCE_INI=$(osmosisd q bank balances $DEPOSITOR $QFLAG | jq -r '.balances[] | select(.denom == "stake") | .amount')
# echo $BALANCE_INI
# execute join swap msg
EXECUTE_JOIN_SWAP_MSG='{"join_swap_extern" : {"pool_id":1, "token_in": {"denom": "stake", "amount":"10"}, "share_out_min_amount":"1"}}'
osmosisd tx wasm execute $CONTRACT_ADDR "$EXECUTE_JOIN_SWAP_MSG" --from $DEPOSITOR $TXFLAG -y \
    --amount 10stake -o json | jq  -r .raw_log

# query for the share amount of the depositor
EXECUTE_DEPOSITOR_SHARE_QUERY='{"depositor_share_amount": {"depositor": "'$DEPOSITOR'"}}'
DEPOSITED_SHARE=$(osmosisd q wasm contract-state smart $CONTRACT_ADDR "$EXECUTE_DEPOSITOR_SHARE_QUERY" $QFLAG | jq -r .data.share_amount)

echo "contract balance $(osmosisd q bank balances $CONTRACT_ADDR $QFLAG)"

BALANCE_BEF=$(osmosisd q bank balances $DEPOSITOR $QFLAG | jq -r '.balances[] | select(.denom == "stake") | .amount')
# echo $BALANCE_BEF
# execute exit swap msg
EXECUTE_EXIT_SWAP_MSG='{"exit_swap_share" : {"pool_id":1, "token_out_denom": "stake", "share_in_amount": "'$DEPOSITED_SHARE'", "token_out_min_amount":"1"}}'
AMOUNT=$DEPOSITED_SHARE'gamm/pool/1'
osmosisd tx wasm execute $CONTRACT_ADDR "$EXECUTE_EXIT_SWAP_MSG" --from validator1 $TXFLAG -y -o json | jq -r .raw_log

BALANCE_AFT=$(osmosisd q bank balances $DEPOSITOR $QFLAG | jq -r '.balances[] | select(.denom == "stake") | .amount')
# echo $BALANCE_AFT
BALANCE_DIF=$(($BALANCE_AFT - $BALANCE_BEF))
echo "Difference between before and after '$BALANCE_DIF'"

echo "contract balance $(osmosisd q bank balances $CONTRACT_ADDR $QFLAG)"

# query for the pool assets
# osmosisd q gamm pool 1 $QFLAG | jq .

# osmosisd q poolincentives gauge-ids 1  $QFLAG

# osmosisd q incentives gauge-by-id 1 $QFLAG

# osmosisd q incentives to-distribute-coins $QFLAG