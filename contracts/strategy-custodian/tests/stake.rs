use crate::helpers::setup;
use cosmwasm_std::{
    coins,
    testing::{mock_env, mock_info},
};
use strategy::v1::msgs::StakeMsg;
use strategy_custodian::{error::ContractError, execute::stake::execute_stake};

mod helpers;

#[test]
fn test_stake() {
    let mut deps = setup();

    // Error: one_coin
    let info = mock_info("anyone", &[]);
    let _ = execute_stake(deps.as_mut(), mock_env(), info, StakeMsg {}).unwrap_err();

    // Error: invalid denom
    let invalid_info = mock_info("anyone", &coins(100, "dummy"));
    let err = execute_stake(deps.as_mut(), mock_env(), invalid_info, StakeMsg {}).unwrap_err();
    assert_eq!(err, ContractError::NoAllowedToken {});

    // Success:
    let info = mock_info("anyone", &coins(100, "denom"));
    let res = execute_stake(deps.as_mut(), mock_env(), info, StakeMsg {}).unwrap();

    assert_eq!(0, res.messages.len());
}
