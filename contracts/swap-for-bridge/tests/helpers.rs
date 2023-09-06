use cosmwasm_schema::serde;

use swap_for_bridge::contract::instantiate;
use swap_for_bridge::contract::query;
use swap_for_bridge::msgs::{InstantiateMsg, QueryMsg};

use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{from_binary, Decimal, Deps, OwnedDeps};

pub fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
    let mut deps = mock_dependencies();

    // instantiate an empty contract
    let info = mock_info(&String::from("anyone"), &[]);

    let instantiate_msg = InstantiateMsg {
        denoms_same_origin: vec!["uatom".to_string(), "ibc/uatom".to_string()],
        authority: info.sender.into(),
        fee_collector: String::from("ununifi1f5vsnrwe7h9dhhyxkwr4yah5u0cke69h03latc"), // totally random. DONT use on mainnet
        fee_rate: Decimal::percent(1),
        min_fee: None,
        max_fee: None,
        lp_fee_weight: Decimal::percent(50),
    };
    let info = mock_info(&String::from("anyone"), &[]);
    let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
    assert_eq!(0, res.messages.len());

    deps
}

pub fn th_query<T: serde::de::DeserializeOwned>(deps: Deps, msg: QueryMsg) -> T {
    from_binary(&query(deps, mock_env(), msg).unwrap()).unwrap()
}
