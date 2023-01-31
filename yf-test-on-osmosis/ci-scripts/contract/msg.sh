#!/bin/bash

set -o errexit -o nounset -u

echo "msg and quereis"

BALANCE_INI=$(osmosisd q bank balances $DEPOSITOR $QFLAG | jq -r '.balances[] | select(.denom == "stake") | .amount')
# execute join swap msg
EXECUTE_JOIN_SWAP_MSG='{"join_swap_extern" : {"token_in": {"denom": "stake", "amount":"100"}, "share_out_min_amount":"1"}}'
osmosisd tx wasm execute $CONTRACT_ADDR "$EXECUTE_JOIN_SWAP_MSG" --from=$DEPOSITOR $TXFLAG -y \
    --amount 100stake -o json | jq  -r .raw_log

# query for the share amount of the depositor
EXECUTE_DEPOSITOR_SHARE_QUERY='{"depositor_share_amount": {"depositor": "'$DEPOSITOR'"}}'
DEPOSITED_SHARE=$(osmosisd q wasm contract-state smart $CONTRACT_ADDR "$EXECUTE_DEPOSITOR_SHARE_QUERY" $QFLAG | jq -r .data.share_amount)
echo "deposited share '$DEPOSITED_SHARE'"
echo "contract balance $(osmosisd q bank balances $CONTRACT_ADDR $QFLAG)"

# swap for earning trading fee
# osmosisd tx gamm swap-exact-amount-in 500uosmo 1 --swap-route-pool-ids=1 --swap-route-denoms=stake --from=faucet $TXFLAG -y
# sleep 7
# osmosisd tx gamm swap-exact-amount-in 500stake 1 --swap-route-pool-ids=1 --swap-route-denoms=uosmo --from=faucet $TXFLAG -y

BALANCE_BEF=$(osmosisd q bank balances $DEPOSITOR $QFLAG | jq -r '.balances[] | select(.denom == "stake") | .amount')
# echo $BALANCE_BEF
# execute exit swap msg
EXECUTE_EXIT_SWAP_MSG='{"exit_swap_share" : {"share_in_amount": "'$DEPOSITED_SHARE'", "token_out_min_amount":"1"}}'
AMOUNT=$DEPOSITED_SHARE'gamm/pool/1'
osmosisd tx wasm execute $CONTRACT_ADDR "$EXECUTE_EXIT_SWAP_MSG" --from=$DEPOSITOR $TXFLAG -y -o json | jq -r .raw_log

BALANCE_AFT=$(osmosisd q bank balances $DEPOSITOR $QFLAG | jq -r '.balances[] | select(.denom == "stake") | .amount')
# echo $BALANCE_AFT
BALANCE_DIF=$(($BALANCE_AFT - $BALANCE_BEF))
echo "Difference between before and after '$BALANCE_DIF'"

echo "contract balance $(osmosisd q bank balances $CONTRACT_ADDR $QFLAG)"

osmosisd q wasm contract-state smart $CONTRACT_ADDR "$EXECUTE_DEPOSITOR_SHARE_QUERY" $QFLAG | jq -r .data.share_amount
# query for the pool assets
# osmosisd q gamm pool 1 $QFLAG | jq .

# osmosisd q poolincentives incentivized-pools $QFLAG

# osmosisd q poolincentives gauge-ids 1  $QFLAG

# osmosisd q incentives gauges $QFLAG -o json | jq .

# osmosisd q incentives to-distribute-coins $QFLAG
