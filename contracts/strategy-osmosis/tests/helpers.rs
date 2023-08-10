use strategy_osmosis::contract::{instantiate, query};
use strategy_osmosis::error::ContractError;
use strategy_osmosis::msgs::{InstantiateMsg, QueryMsg};

use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{from_binary, Deps, DepsMut, OwnedDeps, Response, Uint128};
use strategy_osmosis::state::{Config, State, CONFIG, STATE};
use ununifi_binding::v0::binding::UnunifiMsg;

pub fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
    let mut deps = mock_dependencies();

    // instantiate an empty contract
    let instantiate_msg = InstantiateMsg {
        unbond_period: 0u64,
        transfer_timeout: 300u64,
        controller_deposit_denom: "uguu".to_string(), // this value is not set in the contract as long as "stake" is set no matter what
        quote_denom: "uosmo".to_string(),             // OSMO
        base_denom: "stake".to_string(),
        chain_id: "test-1".to_string(),
        pool_id: 1u64,
        lp_denom: "gamm/pool/1".to_string(),
        transfer_channel_id: "channel-1".to_string(),
        controller_transfer_channel_id: "channel-1".to_string(),
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

#[allow(dead_code)]
pub fn register_ica(
    deps: DepsMut,
    ica_addr: String,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    config.ica_account = ica_addr;
    // save config directly

    CONFIG.save(deps.storage, &config)?;
    Ok(Response::default())
}

#[allow(dead_code)]
pub fn remove_free_atom_from_host_account(deps: DepsMut) {
    let mut state: State = th_query(deps.as_ref(), QueryMsg::State {});
    state.free_base_amount = Uint128::zero();
    _ = STATE.save(deps.storage, &state);
}
