use crate::helpers::setup;
use cosmwasm_std::testing::{mock_env, mock_info};
use helpers::th_query;
use swap_for_bridge::{
    error::ContractError,
    execute::update_params::execute_update_params,
    msgs::{QueryMsg, UpdateParamsMsg},
    types::Params,
};
mod helpers;

#[test]
fn initialized_state() {
    let deps = setup();

    let config: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(2, config.denoms_same_origin.len());
}

#[test]
fn update_config() {
    let mut deps = setup();

    let sender = "authority";
    // Change with other values for further tests
    execute_update_params(
        deps.as_mut(),
        mock_env(),
        mock_info(sender, &[]),
        UpdateParamsMsg {
            authority: Some("authority".to_string()),
            denoms_same_origin: Some(vec![
                "denom1".to_string(),
                "denom2".to_string(),
                "denom3".to_string(),
            ]),
            fee_collector: Some("fee_collector".to_string()),
            fee_rate: None,
            lp_fee_rate: None,
        },
    )
    .unwrap();

    let config: Params = th_query(deps.as_ref(), QueryMsg::Params {});

    assert_eq!(3, config.denoms_same_origin.len());

    let bad_sender = "bad_sender";
    let err = execute_update_params(
        deps.as_mut(),
        mock_env(),
        mock_info(bad_sender, &[]),
        UpdateParamsMsg {
            authority: Some("authority".to_string()),
            denoms_same_origin: Some(vec![
                "denom1".to_string(),
                "denom2".to_string(),
                "denom3".to_string(),
            ]),
            fee_collector: Some("fee_collector".to_string()),
            fee_rate: None,
            lp_fee_rate: None,
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});
}
