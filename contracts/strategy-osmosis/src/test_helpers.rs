#![cfg(test)]

use crate::contract::instantiate;
use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::OwnedDeps;
use strategy_osmosis_interface::strategy::InstantiateMsg;

pub const DEFAULT_TIMEOUT: u64 = 3600; // 1 hour,
pub const CONTRACT_PORT: &str = "ibc:wasm1234567890abcdef";

pub fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
    let mut deps = mock_dependencies();

    // instantiate an empty contract
    let instantiate_msg = InstantiateMsg {
        unbond_period: 0u64,
        deposit_denom: "uguu".to_string(),
    };
    let info = mock_info(&String::from("anyone"), &[]);
    let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
    assert_eq!(0, res.messages.len());

    deps
}
