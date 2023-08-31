use cosmwasm_std::testing::mock_env;
use cosmwasm_std::{coins, IbcEndpoint, Uint128};
use helpers::setup;
use strategy_osmosis::helpers::query_balance;
use strategy_osmosis::msgs::{ChannelInfo, Phase, QueryMsg};
use strategy_osmosis::query::list_channels::query_list_channels;
use strategy_osmosis::query::params::query_params;
use strategy_osmosis::state::{Params, CHANNEL_INFO, PARAMS};

use crate::helpers::th_query;
mod helpers;

// test of query_balance
#[test]
fn test_query_balance() {
    let mut deps = setup();

    let balance = query_balance(
        &deps.as_mut().querier,
        mock_env().contract.address,
        "stake".to_string(),
    )
    .unwrap();
    assert_eq!(balance, Uint128::from(0u128));

    deps.querier
        .update_balance(mock_env().contract.address, coins(1000, "stake"));
    let balance = query_balance(
        &deps.as_mut().querier,
        mock_env().contract.address,
        "stake".to_string(),
    )
    .unwrap();
    assert_eq!(balance, Uint128::from(1000u128));
}

// test of query_params
#[test]
fn test_query_params() {
    let mut deps = setup();

    let params = query_params(deps.as_ref()).unwrap();
    assert_eq!(params.phase, Phase::Deposit);

    let mut params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    params.phase = Phase::Withdraw;
    PARAMS.save(deps.as_mut().storage, &params).unwrap();

    let params = query_params(deps.as_ref()).unwrap();
    assert_eq!(params.phase, Phase::Withdraw);
}

// test of query_list_channels
#[test]
fn test_query_list_channels() {
    let mut deps = setup();

    let res = query_list_channels(deps.as_ref()).unwrap();
    assert_eq!(res.channels.len(), 0);

    // update channel_info
    let channel_info = ChannelInfo {
        id: "channel-1".to_string(),
        counterparty_endpoint: IbcEndpoint {
            port_id: "source_port".to_string(),
            channel_id: "source_channel".to_string(),
        },
        connection_id: "source_port".to_string(),
        address: mock_env().contract.address.to_string(),
    };

    CHANNEL_INFO
        .save(deps.as_mut().storage, &channel_info.id, &channel_info)
        .unwrap();
    let res = query_list_channels(deps.as_ref()).unwrap();
    assert_eq!(res.channels.len(), 1);
    assert_eq!(res.channels[0].id, channel_info.id);
}
