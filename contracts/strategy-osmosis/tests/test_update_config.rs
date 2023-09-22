use cosmwasm_std::testing::{mock_env, mock_info};
use helpers::th_query;
use strategy_osmosis::error::ContractError;
use strategy_osmosis::execute::update_params::execute_update_params;
use strategy_osmosis::msgs::{QueryMsg, UpdateParamsMsg};
use strategy_osmosis::state::Params;

use crate::helpers::setup;

mod helpers;

#[test]
fn initialized_state() {
    let deps = setup();

    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(0, params.unbond_period);
    // assert_eq!("uguu", params.controller_deposit_denom);
}

#[test]
fn update_params() {
    let mut deps = setup();

    let sender = "anyone";
    // Change with other values for further tests
    execute_update_params(
        deps.as_mut(),
        mock_env(),
        mock_info(sender, &[]),
        UpdateParamsMsg {
            authority: None,
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
            superfluid_validator: None,
            automate_superfluid: None,
            extern_tokens: None,
        },
    )
    .unwrap();

    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});

    assert_eq!("uguu", params.controller_deposit_denom);

    let bad_sender = "bad_sender";
    let err = execute_update_params(
        deps.as_mut(),
        mock_env(),
        mock_info(bad_sender, &[]),
        UpdateParamsMsg {
            authority: None,
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
            superfluid_validator: None,
            automate_superfluid: None,
            extern_tokens: None,
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});
}
