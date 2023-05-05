#!/bin/sh

VALIDATOR=$(ununifid keys show -a validator --keyring-backend=test --home=$HOME/.ununifi)
# transactions
ununifid tx wasm store artifacts/strategy_base.wasm --from=validator --keyring-backend=test --chain-id=test --node http://localhost:26657 --gas=auto --gas-adjustment=1.3 -y
ununifid tx wasm instantiate 1 '{"owner":"'$VALIDATOR'", "unbond_period":1, "deposit_denom": "stake"}' --from=validator --label "BaseStrategy" --chain-id=test --gas=auto --gas-adjustment=1.3 -b=sync --keyring-backend=test --no-admin -y
CONTRACT=ununifi14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sm5z28e
ununifid tx wasm execute $CONTRACT '{"stake":{}}' --amount=1000stake --from=validator --gas=auto --gas-adjustment=1.3 --chain-id=test -y --keyring-backend=test
ununifid tx wasm execute $CONTRACT '{"add_rewards":{}}' --amount=1000stake --from=validator --gas=auto --gas-adjustment=1.3 --chain-id=test -y --keyring-backend=test
ununifid tx wasm execute $CONTRACT '{"unstake":{"amount":"1000"}}' --from=validator --gas=auto --gas-adjustment=1.3 --chain-id=test -y --keyring-backend=test
ununifid tx wasm execute $CONTRACT '{"update_config":{"owner":"'$VALIDATOR'","unbond_period":0,"deposit_denom":"stake"}}' --from=validator --gas=auto --gas-adjustment=1.3 --chain-id=test -y --keyring-backend=test 

# proposal txs
STRATEGY_DENOM="stake"
ununifid tx yieldaggregator proposal-add-strategy --title="title" --description="description" --deposit=10000000stake \
 --denom=$STRATEGY_DENOM --contract-addr=$CONTRACT --name="Contract Staking" \
 --git-url="" --from=validator --gas=auto --gas-adjustment=1.3 --chain-id=test -y --keyring-backend=test 

ununifid query gov proposals
ununifid tx gov vote 1 yes --from=validator --gas=auto --gas-adjustment=1.3 --chain-id=test -y --keyring-backend=test 

# create vault
ununifid tx yieldaggregator create-vault $STRATEGY_DENOM \
"0.01" "0.3" 1000000stake 1000000stake "0:1" \
--from=validator --gas=auto --gas-adjustment=1.3 --chain-id=test -y --keyring-backend=test 

# deposit/withdraw txs
ununifid tx yieldaggregator deposit-to-vault 0 20000$STRATEGY_DENOM \
--from=validator --gas=auto --gas-adjustment=1.3 --chain-id=test -y --keyring-backend=test 

ununifid query bank balances $VALIDATOR 

# withdraw 
ununifid tx yieldaggregator withdraw-from-vault 0 10yield-aggregator/vaults/0 \
--from=validator --gas=auto --gas-adjustment=1.3 --chain-id=test -y --keyring-backend=test 

ununifid query bank balances $VALIDATOR  

# queries
ununifid query bank balances $VALIDATOR
ununifid query bank balances $CONTRACT
ununifid query wasm contract-state smart $CONTRACT '{"config":{}}'
ununifid query wasm contract-state smart $CONTRACT '{"unbonding":{"addr": "'$VALIDATOR'"}}'
ununifid query wasm contract-state smart $CONTRACT '{"bonded":{"addr": "'$VALIDATOR'"}}'

ununifid query yieldaggregator list-strategy $STRATEGY_DENOM
ununifid query yieldaggregator list-vault
ununifid query yieldaggregator show-vault 0
ununifid query yieldaggregator params

VAULT=ununifi143d32czv8ruqamnryu6gq4xvdtz442appuapsj
ununifid query wasm contract-state smart $CONTRACT '{"unbonding":{"addr": "'$VAULT'"}}'
ununifid query wasm contract-state smart $CONTRACT '{"bonded":{"addr": "'$VAULT'"}}'
