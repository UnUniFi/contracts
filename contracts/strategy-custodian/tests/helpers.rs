#![cfg(test)]

use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{Decimal, OwnedDeps};
use strategy_custodian::contract::instantiate;
use strategy_custodian::msgs::InstantiateMsg;

pub fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
    let mut deps = mock_dependencies();

    // instantiate an empty contract
    let instantiate_msg = InstantiateMsg {
        deposit_denom: "uguu".to_string(),
        performance_fee_rate: Decimal::zero(),
        withdraw_fee_rate: Decimal::zero(),
        min_withdraw_fee: None,
        max_withdraw_fee: None,
        trusted_kyc_provider_ids: vec![1],
    };
    let info = mock_info(&String::from("anyone"), &[]);
    let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
    assert_eq!(0, res.messages.len());

    deps
}
