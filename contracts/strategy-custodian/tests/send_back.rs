use crate::helpers::setup;
use cosmwasm_std::{
    coins,
    testing::{mock_env, mock_info},
};
use strategy::v1::msgs::StakeMsg;
use strategy_custodian::{
    error::ContractError,
    execute::{send_back::execute_send_back, stake::execute_stake},
    msgs::SendBackMsg,
};

mod helpers;

#[test]
fn test_send_back() {
    let mut deps = setup();

    let info = mock_info("staker", &coins(100, "denom"));
    execute_stake(deps.as_mut(), mock_env(), info, StakeMsg {}).unwrap();

    // Error: fund
    let invalid_info = mock_info("anyone", &[]);
    let _err =
        execute_send_back(deps.as_mut(), mock_env(), invalid_info, SendBackMsg {}).unwrap_err();

    // Error: because of the permission
    let invalid_info = mock_info("anyone", &coins(100, "denom"));
    let err =
        execute_send_back(deps.as_mut(), mock_env(), invalid_info, SendBackMsg {}).unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    // Success:
    let info = mock_info("admin", &coins(100, "denom"));
    let res = execute_send_back(deps.as_mut(), mock_env(), info, SendBackMsg {}).unwrap();

    println!("{:#?}", res.messages);

    assert_eq!(1, res.messages.len());
}
