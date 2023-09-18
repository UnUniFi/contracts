use std::sync::mpsc::Sender;

use crate::helpers::setup;
use cosmwasm_std::{
    coins,
    testing::{mock_env, mock_info}, Uint128,
};
use strategy::v1::msgs::StakeMsg;
use strategy_custodian::{error::ContractError, execute::stake::execute_stake, query::{amounts::query_amounts, share_and_deposit::{query_total_share, query_total_deposit}}};

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

    let total_share = query_total_share(deps.as_ref()).unwrap();
    assert_eq!(Uint128::zero(), total_share.total_share);
    let total_deposit = query_total_deposit(deps.as_ref()).unwrap();
    assert_eq!(Uint128::zero(), total_deposit.total_deopsit);

    // Success:
    let info = mock_info("anyone", &coins(100, "denom"));
    let res = execute_stake(deps.as_mut(), mock_env(), info.clone(), StakeMsg {}).unwrap();
    assert_eq!(0, res.messages.len());

    let sender_amounts = query_amounts(deps.as_ref(), info.sender.into_string()).unwrap();
    assert_eq!(Uint128::new(100), sender_amounts.total_deposited);
    assert_eq!(Uint128::new(0), sender_amounts.bonding_standby);
    assert_eq!(Uint128::new(100), sender_amounts.bonded);
    assert_eq!(Uint128::new(0), sender_amounts.unbonding);

    let total_share = query_total_share(deps.as_ref()).unwrap();
    assert_eq!(Uint128::new(100), total_share.total_share);
    let total_deposit = query_total_deposit(deps.as_ref()).unwrap();
    assert_eq!(Uint128::new(100), total_deposit.total_deopsit);
}
