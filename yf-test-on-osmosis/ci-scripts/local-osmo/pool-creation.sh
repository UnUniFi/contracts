# This script is from github/osmosis-labs/osmosis/scripts/setup-superfluid.sh
set -e
# this script runs under the assumption that a three-validator environment is running on your local machine(multinode-local-testnet.sh)
# this script would do basic setup that has to be achieved to actual superfluid staking
# prior to running this script, have the following json file in the directory running this script

# stake-uosmo.json
# {
# 	"weights": "5stake,5uosmo",
# 	"initial-deposit": "1000000stake,1000000uosmo",
# 	"swap-fee": "0.01",
# 	"exit-fee": "0.01",
# 	"future-governor": "168h"
# }

NODE=http://localhost:26653
CHAIN_ID=osmo-testing
OSMO_HOME=$HOME/.osmosisd/validator1

# create pool
osmosisd tx gamm create-pool --pool-file=./ci-scripts/local-osmo/stake-uosmo.json --from=validator1 --keyring-backend=test --chain-id=$CHAIN_ID --yes --home=$OSMO_HOME --node=$NODE -b block
# sleep 7

# test swap in pool created
# osmosisd tx gamm swap-exact-amount-in 1uosmo 50000 --swap-route-pool-ids=1 --swap-route-denoms=stake --from=validator1 --keyring-backend=test --chain-id=$CHAIN_ID --yes --home=$HOME/.osmosisd/validator1 --node $NODE
# sleep 7

# create a lock up with lockable duration 360h
# osmosisd tx lockup lock-tokens 10000000000000000000gamm/pool/1 --duration=360h --from=validator1 --keyring-backend=test --chain-id=$CHAIN_ID --broadcast-mode=block --yes --home=$HOME/.osmosisd/validator1 --node $NODE
# sleep 7
