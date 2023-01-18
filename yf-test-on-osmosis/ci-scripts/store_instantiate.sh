#!/bin/bash
set -e
source ./ci-scripts/local-osmo/env

CONTRACT_NAME="yf_test_on_osmosis-aarch64"
NODE=http://localhost:26653
CHAIN_ID=osmo-testing
OSMO_HOME=$HOME/.osmosisd/validator1
TXFLAG="--gas=auto --gas-prices 0.1uosmo --gas-adjustment 1.3 -b block --keyring-backend=test --home=$OSMO_HOME --node $NODE --chain-id $CHAIN_ID"
LABEL="yf_test_on_osmosis"

# store bytecode and acquire code id of it
RES=$(osmosisd tx wasm store "./artifacts/$CONTRACT_NAME.wasm" --from validator1 \
    -y --output json $TXFLAG)
# echo $RES
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')
echo $CODE_ID

# instantiate contract out of the uploaded code using that code id
# write instance state in josn 
INIT='{}'

osmosisd tx wasm instantiate $CODE_ID "$INIT" --from validator1 \
    $TXFLAG -y --no-admin --label $LABEL

