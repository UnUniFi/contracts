use crate::helpers::setup;
use cosmwasm_std::testing::{mock_env, mock_info};
use kyc::error::ContractError;
use kyc::execute::update_params::execute_update_params;
use kyc::msgs::UpdateParamsMsg;
use kyc::query::params::query_params;

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
            authority: Some("authority2".to_string()),
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    let params = query_params(deps.as_ref()).unwrap();
    assert_eq!(params.authority.as_str(), "authority");

    let invalid_info = mock_info("authority", &[]);
    execute_update_params(
        deps.as_mut(),
        mock_env(),
        invalid_info,
        UpdateParamsMsg {
            authority: Some("authority2".to_string()),
        },
    )
    .unwrap();

    let params = query_params(deps.as_ref()).unwrap();
    assert_eq!(params.authority.as_str(), "authority2");
}
