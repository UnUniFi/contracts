use cosmwasm_std::testing::{mock_env, mock_info};
use helpers::th_query;
use strategy_osmosis::error::ContractError;
use strategy_osmosis::execute::update_config::execute_update_config;
use strategy_osmosis::msgs::{QueryMsg, UpdateConfigMsg};
use strategy_osmosis::state::Config;

use crate::helpers::setup;

mod helpers;

#[test]
fn initialized_state() {
    let deps = setup();

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(0, config.unbond_period);
    // assert_eq!("uguu", config.controller_deposit_denom);
}

#[test]
fn update_config() {
    let mut deps = setup();

    let sender = "anyone";
    // Change with other values for further tests
    execute_update_config(
        deps.as_mut(),
        mock_env(),
        mock_info(sender, &[]),
        UpdateConfigMsg {
            owner: None,
            deposit_token: None,
            unbond_period: None,
            lp_denom: None,
            ica_channel_id: None,
            phase: None,
            phase_step: None,
            transfer_timeout: None,
            transfer_channel_id: None,
            quote_denom: None,
            base_denom: None,
            controller_transfer_channel_id: None,
            controller_deposit_denom: Some("uguu".to_string()),
            pool_id: None,
            chain_id: None,
        },
    )
    .unwrap();

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});

    assert_eq!("uguu", config.controller_deposit_denom);

    let bad_sender = "bad_sender";
    let err = execute_update_config(
        deps.as_mut(),
        mock_env(),
        mock_info(bad_sender, &[]),
        UpdateConfigMsg {
            owner: None,
            deposit_token: None,
            unbond_period: None,
            lp_denom: None,
            ica_channel_id: None,
            phase: None,
            phase_step: None,
            transfer_timeout: None,
            transfer_channel_id: None,
            quote_denom: None,
            base_denom: None,
            controller_transfer_channel_id: None,
            controller_deposit_denom: Some("stake".to_string()),
            pool_id: None,
            chain_id: None,
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});
}
