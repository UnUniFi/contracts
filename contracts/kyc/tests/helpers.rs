#![cfg(test)]

use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{from_binary, Deps, OwnedDeps};
use kyc::contract::{instantiate, query};
use kyc::msgs::{InstantiateMsg, QueryMsg};

pub fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
    let mut deps = mock_dependencies();

    // instantiate an empty contract
    let instantiate_msg = InstantiateMsg {
        authority: String::from("authority"),
    };
    let info = mock_info(&String::from("authority"), &[]);
    let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
    assert_eq!(0, res.messages.len());

    deps
}

#[allow(dead_code)]
pub fn th_query<T: cosmwasm_schema::serde::de::DeserializeOwned>(deps: Deps, msg: QueryMsg) -> T {
    from_binary(&query(deps, mock_env(), msg).unwrap()).unwrap()
}
