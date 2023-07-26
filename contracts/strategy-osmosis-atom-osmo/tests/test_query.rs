use cosmwasm_std::testing::mock_env;
use cosmwasm_std::{Uint128, Addr, Api, CosmosMsg, Binary, coins};
use helpers::setup;
use strategy_osmosis::strategy::{Phase, QueryMsg, ChannelInfo};
use strategy_osmosis_atom_osmo::binding::UnunifiMsg;
use strategy_osmosis_atom_osmo::helpers::decode_and_convert;
use strategy_osmosis_atom_osmo::ica::determine_ica_amounts;
use strategy_osmosis_atom_osmo::icq::{submit_icq_for_host, create_account_denom_balance_key, create_pool_key};
use strategy_osmosis_atom_osmo::query::{query_balance, query_config, query_list_channels};
use strategy_osmosis_atom_osmo::state::{Config, STAKE_RATE_MULTIPLIER, HostConfig, ControllerConfig, CONFIG};

use crate::helpers::th_query;
mod helpers;

// test of query_balance
#[test]
fn test_query_balance() {
    let mut deps = setup();
    
    let balance = query_balance(&deps.as_mut().querier, mock_env().contract.address, "stake".to_string()).unwrap();
    assert_eq!(balance, Uint128::from(0u128));

    deps.querier.update_balance(
        mock_env().contract.address,
        coins(1000, "stake"),
    );
    let balance = query_balance(&deps.as_mut().querier, mock_env().contract.address, "stake".to_string()).unwrap();
    assert_eq!(balance, Uint128::from(1000u128));
}

// test of query_config
#[test]
fn test_query_config() {
    let mut deps = setup();
    
    let config = query_config(deps.as_ref()).unwrap();
    assert_eq!(config.phase, Phase::Deposit);

    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    config.phase = Phase::Withdraw;
    CONFIG.save(deps.as_mut().storage, &config).unwrap();

    let config = query_config(deps.as_ref()).unwrap();
    assert_eq!(config.phase, Phase::Withdraw);
}
