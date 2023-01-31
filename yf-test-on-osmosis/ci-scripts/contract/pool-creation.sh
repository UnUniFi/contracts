# This script is from github/osmosis-labs/osmosis/scripts/setup-superfluid.sh
set -e
# this script runs under the assumption that a three-validator environment is running on your local machine(multinode-local-testnet.sh)
# this script would do basic setup that has to be achieved to actual superfluid staking
# prior to running this script, have the following json file in the directory running this script

echo "creating pool"

CREATION_POOL=$SCRIPT_DIR/stake-uosmo.json
# create pool
osmosisd tx gamm create-pool --pool-file=$CREATION_POOL --pool-type=stableswap --from=$DEPOSITOR --keyring-backend=test --chain-id=$CHAIN_ID --yes --home=$OSMO_HOME --node=$NODE -b block -o json | jq '.height'
# sleep 7

# create a lock up with lockable duration 360h
# osmosisd tx lockup lock-tokens 10000000000000000000gamm/pool/1 --duration=360h --from=validator1 --keyring-backend=test --chain-id=$CHAIN_ID --broadcast-mode=block --yes --home=$HOME/.osmosisd/validator1 --node $NODE
# sleep 7
