use osmosis_std::types::osmosis::gamm::v1beta1::MsgJoinPool;
use prost::EncodeError;
use prost_types::Any;
use proto::traits::MessageExt;
use strategy::error::ContractError;
use strategy_osmosis::strategy::{InstantiateMsg, QueryMsg, UpdateConfigMsg};
use strategy_osmosis_atom_osmo::binding::UnunifiMsg;
use strategy_osmosis_atom_osmo::contract::{execute_update_config, instantiate, query};

use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{
    from_binary,
    // testing::{MockApi, MockStorage},
    Addr,
    BankMsg,
    Coin,
    CosmosMsg,
    Decimal,
    Deps,
    DepsMut,
    Env,
    Event,
    MessageInfo,
    OwnedDeps,
    Response,
    StdResult,
    Uint128,
};
use strategy_osmosis_atom_osmo::state::{Config, CONFIG};

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

pub fn register_ica(
    deps: DepsMut,
    ica_addr: String,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    config.ica_account = ica_addr;
    // save config directly

    CONFIG.save(deps.storage, &config)?;
    Ok((Response::default()))
}

pub fn remove_free_atom_from_host_account(deps: DepsMut) {
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    config.host_config.free_base_amount = Uint128::zero();
    CONFIG.save(deps.storage, &config);
}
