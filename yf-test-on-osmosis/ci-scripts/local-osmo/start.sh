#!/bin/bash

## execute under tests folder

set -e

killall osmosisd || true
rm -rf $HOME/.osmosisd/

SCRIPT_DIR=$(cd $(dirname $0); pwd)
source $SCRIPT_DIR/env

mkdir -p $OSMO_HOME

osmosisd init --chain-id=$CHAIN_ID validator1 --home=$OSMO_HOME;
osmosisd keys add validator1  --keyring-backend=test --home=$OSMO_HOME --recover < $SCRIPT_DIR/validator1
osmosisd keys add faucet --recover < $SCRIPT_DIR/faucet --keyring-backend=test --home=$OSMO_HOME

update_genesis () {    
    cat $OSMO_HOME/config/genesis.json | jq "$1" > $OSMO_HOME/config/tmp_genesis.json && mv $OSMO_HOME/config/tmp_genesis.json $OSMO_HOME/config/genesis.json
}
sed -i -o 's/stake/uosmo/g' $OSMO_HOME/config/genesis.json

# change staking denom to uosmo
update_genesis '.app_state["staking"]["params"]["bond_denom"]="uosmo"'

# create validator node with tokens to transfer to the three other nodes
osmosisd add-genesis-account $(osmosisd keys show validator1 -a --keyring-backend=test --home=$OSMO_HOME) 100000000000uosmo,100000000000stake --home=$OSMO_HOME
osmosisd add-genesis-account $(osmosisd keys show faucet -a --keyring-backend=test --home=$OSMO_HOME) 100000000000uosmo,100000000000stake --home=$OSMO_HOME
osmosisd gentx validator1 500000000uosmo --keyring-backend=test --home=$OSMO_HOME --chain-id=$CHAIN_ID
osmosisd collect-gentxs --home=$OSMO_HOME

# update crisis variable to uosmo
update_genesis '.app_state["crisis"]["constant_fee"]["denom"]="uosmo"'

# udpate gov genesis
update_genesis '.app_state["gov"]["voting_params"]["voting_period"]="60s"'
update_genesis '.app_state["gov"]["deposit_params"]["min_deposit"][0]["denom"]="uosmo"'

# update epochs genesis
update_genesis '.app_state["epochs"]["epochs"][0]["identifier"]="min"'
update_genesis '.app_state["epochs"]["epochs"][0]["duration"]="60s"'

# update poolincentives genesis
update_genesis '.app_state["poolincentives"]["lockable_durations"][0]="120s"'
update_genesis '.app_state["poolincentives"]["lockable_durations"][1]="180s"'
update_genesis '.app_state["poolincentives"]["lockable_durations"][2]="240s"'
update_genesis '.app_state["poolincentives"]["params"]["minted_denom"]="uosmo"'

# update incentives genesis
update_genesis '.app_state["incentives"]["params"]["distr_epoch_identifier"]="min"'
update_genesis '.app_state["incentives"]["lockable_durations"][0]="1s"'
update_genesis '.app_state["incentives"]["lockable_durations"][1]="120s"'
update_genesis '.app_state["incentives"]["lockable_durations"][2]="180s"'
update_genesis '.app_state["incentives"]["lockable_durations"][3]="240s"'

# update mint genesis
update_genesis '.app_state["mint"]["params"]["epoch_identifier"]="min"'

# # update epochs genesis
# update_genesis '.app_state["epochs"]["epochs"][0]["duration"]="60s"'
# update_genesis '.app_state["epochs"]["epochs"][0]["identifier"]="minute"'

# # update poolincentives genesis
# update_genesis '.app_state["poolincentives"]["lockable_durations"][0]="60s"'
# update_genesis '.app_state["poolincentives"]["lockable_durations"][1]="180s"'
# update_genesis '.app_state["poolincentives"]["lockable_durations"][2]="240s"'
# update_genesis '.app_state["poolincentives"]["params"]["minted_denom"]="uosmo"'

# # update incentives genesis
# update_genesis '.app_state["incentives"]["lockable_durations"][0]="1s"'
# update_genesis '.app_state["incentives"]["lockable_durations"][1]="120s"'
# update_genesis '.app_state["incentives"]["lockable_durations"][2]="180s"'
# update_genesis '.app_state["incentives"]["lockable_durations"][3]="240s"'
# update_genesis '.app_state["incentives"]["params"]["distr_epoch_identifier"]="minute"'

# update mint genesis
update_genesis '.app_state["mint"]["params"]["mint_denom"]="uosmo"'
# update_genesis '.app_state["mint"]["params"]["epoch_identifier"]="minute"'

# update gamm genesis
update_genesis '.app_state["gamm"]["params"]["pool_creation_fee"][0]["denom"]="uosmo"'

VALIDATOR1_CONFIG=$OSMO_HOME/config/config.toml
sed -i -E "s|tcp://127.0.0.1:26657|$NODE|g" $VALIDATOR1_CONFIG

# tmux new -s validator1 -d 
osmosisd start --home=$OSMO_HOME --minimum-gas-prices=0uosmo

# sleep 5

# echo "creating a pool using stake-uosmo.json"
# $RELATIVE_SCRIPT_PATH_FROM_TEST/pool-creation.sh
