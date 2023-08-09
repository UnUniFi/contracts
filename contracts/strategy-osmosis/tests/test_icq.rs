use cosmwasm_std::testing::mock_env;
use cosmwasm_std::{Binary, CosmosMsg};
use helpers::setup;
use strategy_osmosis::helpers::decode_and_convert;
use strategy_osmosis::icq::{
    create_account_denom_balance_key, create_pool_key, submit_icq_for_host,
};
use strategy_osmosis::msgs::QueryMsg;
use strategy_osmosis::state::{Config, CONFIG};
use ununifi_binding::v0::binding::UnunifiMsg;

use crate::helpers::th_query;
mod helpers;

#[test]
fn test_submit_icq_for_host() {
    let mut deps = setup();
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    config.ica_account =
        "osmo1aqvlxpk8dc4m2nkmxkf63a5zez9jkzgm6amkgddhfk0qj9j4rw3q662wuk".to_string();

    CONFIG.save(deps.as_mut().storage, &config).unwrap();

    let res = submit_icq_for_host(deps.as_mut().storage, mock_env());
    assert!(res.is_ok());
    assert_eq!(res.as_ref().unwrap().messages.len(), 4);

    // check the query_key is set properly for each
    let config = CONFIG.load(deps.as_ref().storage).unwrap();
    assert_eq!(config.pending_icq, 4u64);

    let converted_addr_bytes = decode_and_convert(&config.ica_account.as_str()).unwrap();

    let exp_base_balance_key = create_account_denom_balance_key(
        converted_addr_bytes.clone(),
        config.host_config.base_denom.to_string(),
    )
    .unwrap();
    let exp_quote_balance_key = create_account_denom_balance_key(
        converted_addr_bytes.clone(),
        config.host_config.quote_denom.to_string(),
    )
    .unwrap();
    let exp_lp_balance_key = create_account_denom_balance_key(
        converted_addr_bytes.clone(),
        config.host_config.lp_denom.to_string(),
    )
    .unwrap();
    let exp_pool_key = create_pool_key(config.host_config.pool_id).unwrap();

    let mut i = 0;
    for message in res.as_ref().unwrap().messages.clone() {
        if let CosmosMsg::Custom(UnunifiMsg::SubmitICQRequest {
            chain_id,
            connection_id,
            query_prefix,
            query_key,
        }) = &message.msg
        {
            if i == 0 {
                assert_eq!(query_key, &Binary(exp_base_balance_key.clone()));
            }
            if i == 1 {
                assert_eq!(query_key, &Binary(exp_quote_balance_key.clone()));
            }
            if i == 2 {
                assert_eq!(query_key, &Binary(exp_lp_balance_key.clone()));
            }
            if i == 3 {
                assert_eq!(query_key, &Binary(exp_pool_key.clone()));
            }
        }
        i += 1;
    }
}
