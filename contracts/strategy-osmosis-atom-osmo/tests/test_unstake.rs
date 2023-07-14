use cosmwasm_std::{coins, Uint128};
use cosmwasm_std::testing::{mock_env, mock_info};
use helpers::th_query;
use strategy::error::ContractError;
use strategy::strategy::{ExecuteMsg, StakeMsg};
use strategy_osmosis_atom_osmo::contract::{execute_update_config, execute_stake, execute_unstake};
use strategy_osmosis_atom_osmo::msg::{UpdateConfigMsg, QueryMsg};
use strategy_osmosis_atom_osmo::state::{Config};

use crate::helpers::setup;

mod helpers;

#[test]
fn unstake() {
    let mut deps = setup();
    let sender = "anyone";

    // Error: because of no deposit token
    let info = mock_info(sender, &coins(10000 as u128, "stake"));
    let err = execute_unstake(deps.as_mut(), info.funds[0].amount.clone(), info.sender).unwrap();
    println!("{:?}", err);
    // assert_eq!();

    // Error: because of insufficient deposit
    let stake_info = mock_info(sender, &coins(100 as u128, "stake"));
    execute_stake(deps.as_mut(), mock_env(), stake_info.funds[0].clone(), stake_info.sender);
    let unstake_info = mock_info(sender, &coins(10000 as u128, "stake"));
    let err = execute_unstake(deps.as_mut(), unstake_info.funds[0].amount.clone(), unstake_info.sender).unwrap();
    // Below should be true in the future
    // assert_eq!(err, ContractError::AmountOverflow {  });

    // Success:

    // stake first
    let stake_info = mock_info(sender, &coins(10000 as u128, "stake"));
    execute_stake(deps.as_mut(), mock_env(), stake_info.funds[0].clone(), stake_info.sender);
    
    let unstake_info = mock_info(sender, &coins(100 as u128, "stake"));
    let res = execute_unstake(deps.as_mut(), unstake_info.funds[0].amount.clone(), unstake_info.sender).unwrap();
    assert_eq!(100, res.attributes[1].value.parse::<u128>().unwrap());
}