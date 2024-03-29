use crate::helpers::setup;
use cosmwasm_std::{
    coins,
    testing::{mock_env, mock_info},
    Uint128,
};
use strategy::v1::msgs::{StakeMsg, UnstakeMsg};
use strategy_custodian::{execute::{stake::execute_stake, unstake::execute_unstake}, query::amounts::query_amounts};

mod helpers;

#[test]
fn test_unstake() {
    let mut deps = setup();

    let info = mock_info("staker", &coins(100, "denom"));
    execute_stake(deps.as_mut(), mock_env(), info, StakeMsg {}).unwrap();

    // Error: different sender
    let invalid_info = mock_info("anyone", &[]);
    let _err = execute_unstake(
        deps.as_mut(),
        mock_env(),
        invalid_info,
        UnstakeMsg {
            share_amount: Uint128::new(100u128),
            recipient: None,
        },
    )
    .unwrap_err();
    
    // Success:
    let info = mock_info("staker", &[]);
    let res = execute_unstake(
        deps.as_mut(),
        mock_env(),
        info,
        UnstakeMsg {
            share_amount: Uint128::new(50u128),
            recipient: None,
        },
    )
    .unwrap();

    assert_eq!(0, res.messages.len());

    // Success:
    let info = mock_info("staker", &[]);
    let res = execute_unstake(
        deps.as_mut(),
        mock_env(),
        info.clone(),
        UnstakeMsg {
            share_amount: Uint128::new(50u128),
            recipient: Some("recipient".to_string()),
        },
    )
    .unwrap();

    assert_eq!(0, res.messages.len());

    let staker_amounts = query_amounts(deps.as_ref(), info.sender.into_string()).unwrap();
    assert_eq!(Uint128::new(50), staker_amounts.total_deposited);
    assert_eq!(Uint128::zero(), staker_amounts.bonding_standby);
    assert_eq!(Uint128::new(50), staker_amounts.unbonding);
    assert_eq!(Uint128::zero(), staker_amounts.bonded);

    let recipient_amounts = query_amounts(deps.as_ref(), "recipient".to_string()).unwrap();
    assert_eq!(Uint128::new(50), recipient_amounts.total_deposited);
    assert_eq!(Uint128::zero(), recipient_amounts.bonding_standby);
    assert_eq!(Uint128::new(50), recipient_amounts.unbonding);
    assert_eq!(Uint128::zero(), recipient_amounts.bonded);
}
