use strategy_osmosis_atom_osmo::contract::{instantiate, query};
use strategy_osmosis_atom_osmo::msg::{InstantiateMsg, QueryMsg};

use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{
    from_binary,
    // testing::{MockApi, MockStorage},
    Addr, Coin, Decimal, Deps, DepsMut, Event, OwnedDeps, Uint128,
};

pub const DEFAULT_TIMEOUT: u64 = 3600; // 1 hour,
pub const CONTRACT_PORT: &str = "ibc:wasm1234567890abcdef";


pub fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
    let mut deps = mock_dependencies();

    // instantiate an empty contract
    let instantiate_msg = InstantiateMsg {
        unbond_period: 0u64,
        deposit_denom: "uguu".to_string(), // this value is not set in the contract as long as "stake" is set no matter what
    };
    let info = mock_info(&String::from("anyone"), &[]);
    let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
    assert_eq!(0, res.messages.len());

    deps
}

pub fn th_query<T: serde::de::DeserializeOwned>(deps: Deps, msg: QueryMsg) -> T {
    from_binary(&query(deps, mock_env(), msg).unwrap()).unwrap()
}
