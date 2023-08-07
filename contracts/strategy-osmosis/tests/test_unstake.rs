use cosmwasm_std::{coins, OverflowError, StdError, Uint128};
// use cosmwasm_std::Overflow;
use cosmwasm_std::testing::{mock_env, mock_info};
use helpers::th_query;
use strategy::v0::msgs::{ExecuteMsg, StakeMsg};
use strategy_osmosis::contract::{execute_stake, execute_unstake, execute_update_config};
use strategy_osmosis::error::{ContractError, NoDeposit};
use strategy_osmosis::msgs::{QueryMsg, UpdateConfigMsg};
use strategy_osmosis::state::Config;

use crate::helpers::setup;

mod helpers;

#[test]
fn unstake() {
    let mut deps = setup();
    let sender = "anyone";

    // Error: because of no deposit token
    let info = mock_info(sender, &coins(10000 as u128, "uguu"));
    let err =
        execute_unstake(deps.as_mut(), info.funds[0].amount.clone(), info.sender).unwrap_err();
    assert_eq!(err, ContractError::Std(NoDeposit {}.into()));

    // Error: because of insufficient deposit
    let stake_info = mock_info(sender, &coins(100 as u128, "uguu"));
    execute_stake(
        deps.as_mut(),
        mock_env(),
        stake_info.funds[0].clone(),
        stake_info.sender,
    );
    let unstake_info = mock_info(sender, &coins(10000 as u128, "uguu"));
    let result = execute_unstake(
        deps.as_mut(),
        unstake_info.funds[0].amount.clone(),
        unstake_info.sender,
    );
    assert!(result.is_err(), "overflow");

    // Success:
    let unstake_info = mock_info(sender, &coins(10 as u128, "uguu"));
    let res = execute_unstake(
        deps.as_mut(),
        unstake_info.funds[0].amount.clone(),
        unstake_info.sender,
    )
    .unwrap();
    assert_eq!(10, res.attributes[1].value.parse::<u128>().unwrap());
}
