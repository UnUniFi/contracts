use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{from_binary, Decimal, Deps, OwnedDeps};
use swap_for_bridge::contract::{instantiate, query};
use swap_for_bridge::msgs::{InstantiateMsg, QueryMsg};

pub fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
    let mut deps = mock_dependencies();

    let instantiate_msg = InstantiateMsg {
        authority: "authority".to_string(),
        denoms_same_origin: vec!["denom1".to_string(), "denom2".to_string()],
        fee_collector: "fee_collector".to_string(),
        fee_rate: Decimal::percent(1),
        lp_fee_rate: Decimal::percent(1),
    };
    let info = mock_info(&String::from("anyone"), &[]);
    let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
    assert_eq!(0, res.messages.len());

    deps
}

#[allow(dead_code)]
pub fn th_query<T: cosmwasm_schema::serde::de::DeserializeOwned>(deps: Deps, msg: QueryMsg) -> T {
    from_binary(&query(deps, mock_env(), msg).unwrap()).unwrap()
}
