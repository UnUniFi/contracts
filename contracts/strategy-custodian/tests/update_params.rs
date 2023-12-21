use crate::helpers::setup;
use cosmwasm_std::testing::{mock_env, mock_info};
use strategy_custodian::{
    error::ContractError, execute::update_params::execute_update_params, msgs::UpdateParamsMsg,
    query::params::query_params,
};

mod helpers;

#[test]
fn test_update_params() {
    let mut deps = setup();

    // Error: because of the permission
    let invalid_info = mock_info("anyone", &[]);
    let err = execute_update_params(
        deps.as_mut(),
        mock_env(),
        invalid_info,
        UpdateParamsMsg {
            admin: Some("admin2".to_string()),
            deposit_denom: None,
            performance_fee_rate: None,
            withdraw_fee_rate: None,
            min_withdraw_fee: None,
            max_withdraw_fee: None,
            trusted_kyc_provider_ids: None,
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    let params = query_params(deps.as_ref()).unwrap();
    assert_eq!(params.admin.as_str(), "admin");

    // Success
    let info = mock_info("admin", &[]);
    execute_update_params(
        deps.as_mut(),
        mock_env(),
        info.clone(),
        UpdateParamsMsg {
            admin: Some("admin2".to_string()),
            deposit_denom: Some("denom2".to_string()),
            performance_fee_rate: None,
            withdraw_fee_rate: None,
            min_withdraw_fee: None,
            max_withdraw_fee: None,
            trusted_kyc_provider_ids: None,
        },
    )
    .unwrap();

    let params = query_params(deps.as_ref()).unwrap();
    assert_eq!(params.admin.as_str(), "admin2");
    assert_eq!(params.deposit_denom.as_str(), "denom2");
}
