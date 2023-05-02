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

# queries
ununifid query bank balances $VALIDATOR
ununifid query bank balances $CONTRACT
ununifid query wasm contract-state smart $CONTRACT '{"config":{}}'
ununifid query wasm contract-state smart $CONTRACT '{"unbonding":{"addr": "'$VALIDATOR'"}}'
ununifid query wasm contract-state smart $CONTRACT '{"bonded":{"addr": "'$VALIDATOR'"}}'
