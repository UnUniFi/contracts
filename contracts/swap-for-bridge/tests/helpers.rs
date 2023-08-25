use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{Decimal, OwnedDeps};
use swap_for_bridge::contract::instantiate;
use swap_for_bridge::msgs::InstantiateMsg;
use swap_for_bridge::types::FeeConfig;

pub fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
    let mut deps = mock_dependencies();

    let instantiate_msg = InstantiateMsg {
        authority: "authority".to_string(),
        treasury: "treasury".to_string(),
        denoms_same_origin: vec!["denom1".to_string(), "denom2".to_string()],
        fee: FeeConfig {
            commission_rate: Decimal::from_atomics(1u128, 3).unwrap(),
            lp_fee_weight: Decimal::from_atomics(5u128, 1).unwrap(),
        },
    };
    let info = mock_info(&String::from("anyone"), &[]);
    let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
    assert_eq!(0, res.messages.len());

    deps
}
