use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{coins, Uint128};
use strategy::v1::msgs::{StakeMsg, UnstakeMsg};
use strategy_osmosis::error::{ContractError, NoDeposit};
use strategy_osmosis::execute::stake::execute_stake;
use strategy_osmosis::execute::unstake::execute_unstake;

use crate::helpers::setup;

mod helpers;

#[test]
fn unstake() {
    let mut deps = setup();
    let sender = "anyone";

    // Error: because of no deposit token
    let info = mock_info(sender, &coins(10000 as u128, "uguu"));
    let err = execute_unstake(
        deps.as_mut(),
        mock_env(),
        info,
        UnstakeMsg {
            share_amount: Uint128::from(10000u128),
            recipient: None,
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::Std(NoDeposit {}.into()));

    // Error: because of insufficient deposit
    let stake_info = mock_info(sender, &coins(100 as u128, "uguu"));
    _ = execute_stake(deps.as_mut(), mock_env(), stake_info, StakeMsg {});
    let unstake_info = mock_info(sender, &coins(10000 as u128, "uguu"));
    let result = execute_unstake(
        deps.as_mut(),
        mock_env(),
        unstake_info,
        UnstakeMsg {
            share_amount: Uint128::from(10000u128),
            recipient: None,
        },
    );
    assert!(result.is_err(), "overflow");

    // Success:
    let unstake_info = mock_info(sender, &coins(10 as u128, "uguu"));
    let res = execute_unstake(
        deps.as_mut(),
        mock_env(),
        unstake_info,
        UnstakeMsg {
            share_amount: Uint128::from(10u128),
            recipient: None,
        },
    )
    .unwrap();
    assert_eq!(10, res.attributes[3].value.parse::<u128>().unwrap());
}
