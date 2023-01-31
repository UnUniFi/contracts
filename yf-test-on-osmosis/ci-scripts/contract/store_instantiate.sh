#!/bin/bash
set -e

echo "Storing and instantiating contract"

# store bytecode and acquire code id of it
RES=$(osmosisd tx wasm store "$SCRIPT_DIR/../../artifacts/$CONTRACT_NAME.wasm" --from=$DEPOSITOR -y --output json $TXFLAG)
# echo $RES
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')
echo $CODE_ID

# instantiate contract out of the uploaded code using that code id
# write instance state in josn 
INIT='{"pool_id":1, "deposit_token_denom": "stake"}'

CONTRACT_ADDR=$(osmosisd tx wasm instantiate $CODE_ID "$INIT" --from=$DEPOSITOR $TXFLAG -y --no-admin --label $LABEL -o json | jq -r '.logs[0].events[0].attributes[0].value')
echo $CONTRACT_ADDR