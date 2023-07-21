use strategy::error::ContractError;
use strategy_osmosis_atom_osmo::binding::UnunifiMsg;
use strategy_osmosis_atom_osmo::contract::{instantiate, query, execute_update_config};
use strategy_osmosis_atom_osmo::msg::{InstantiateMsg, QueryMsg, UpdateConfigMsg};

use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{
    from_binary,
    // testing::{MockApi, MockStorage},
    Addr, Coin, Decimal, Deps, DepsMut, Event, OwnedDeps, Uint128, Response, Env, MessageInfo, StdResult, CosmosMsg, BankMsg,
};
use strategy_osmosis_atom_osmo::state::{CONFIG, Config};

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

pub fn register_ica(deps: DepsMut, ica_addr: String) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    config.ica_account = ica_addr;
    // save config directly

    CONFIG.save(deps.storage, &config)?;
    Ok((Response::default()))
}

pub fn send_funds_to_contract(
    env: Env,
    amount: Vec<Coin>,
) -> StdResult<Response> {
    let send_msg = CosmosMsg::Bank(BankMsg::Send { 
        to_address: env.contract.address.to_string(),
        amount: amount,
    });
    
    Ok(Response::new().add_message(send_msg))
}

pub fn remove_free_atom_from_host_account(
    deps: DepsMut,
) {
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    config.host_config.free_atom_amount = Uint128::zero();
    CONFIG.save(deps.storage, &config);
}

