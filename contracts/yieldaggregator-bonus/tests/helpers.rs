use std::collections::BTreeMap;

use cosmwasm_schema::serde;
use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{from_binary, Deps, OwnedDeps};
use yieldaggregator_adapter::contract::{instantiate, query};
use yieldaggregator_adapter::msgs::{InstantiateMsg, QueryMsg};

pub fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
    let mut deps = mock_dependencies();

    // instantiate an empty contract
    let info = mock_info(&String::from("anyone"), &[]);

    let instantiate_msg = InstantiateMsg {
        authority: "authority".to_string(),
        denom_swap_contract_map: vec![("denom1".to_string(), "contract1".to_string())]
            .into_iter()
            .collect::<BTreeMap<_, _>>(),
    };
    let info = mock_info(&String::from("anyone"), &[]);
    let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
    assert_eq!(0, res.messages.len());

    deps
}

pub fn th_query<T: serde::de::DeserializeOwned>(deps: Deps, msg: QueryMsg) -> T {
    from_binary(&query(deps, mock_env(), msg).unwrap()).unwrap()
}
