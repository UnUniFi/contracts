#![cfg(test)]

use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{Decimal, OwnedDeps};
use nfloan::contract::instantiate;
use nfloan::msgs::InstantiateMsg;

pub fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
    let mut deps = mock_dependencies();

    // instantiate an empty contract
    let instantiate_msg = InstantiateMsg {
        authority: "authority".to_string(),
        fee_collector: "fee_collector".to_string(),
        selling_fee_rate: Decimal::percent(1),
        interest_fee_rate: Decimal::percent(1),
    };
    let info = mock_info(&String::from("anyone"), &[]);
    let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
    assert_eq!(0, res.messages.len());

    deps
}
