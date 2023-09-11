use crate::helpers::setup;
use cosmwasm_std::coin;
use cosmwasm_std::testing::{mock_env, mock_info};
use helpers::th_query;
use kyc::error::ContractError;
use kyc::execute::register_provider::execute_register_provider;
use kyc::execute::update_provider::execute_update_provider;
use kyc::msgs::{QueryMsg, RegisterProviderMsg, UpdateProviderMsg};
use kyc::types::Provider;

mod helpers;

#[test]
fn test_update_provider() {
    let mut deps = setup();

    let info = mock_info("authority", &[]);
    execute_register_provider(
        deps.as_mut(),
        mock_env(),
        info,
        RegisterProviderMsg {
            address: "provider".to_string(),
            name: "anyone".to_string(),
            identity: "anyone".to_string(),
            website: "anyone".to_string(),
            security_contact: "anyone".to_string(),
            details: "anyone".to_string(),
            information_fee: coin(0, "denom"),
        },
    )
    .unwrap();

    // Error: because of the permission
    let invalid_info = mock_info("anyone", &[]);
    let err = execute_update_provider(
        deps.as_mut(),
        mock_env(),
        invalid_info,
        UpdateProviderMsg {
            id: 0,
            address: None,
            name: Some("new_name".to_string()),
            identity: None,
            website: None,
            security_contact: None,
            details: None,
            information_fee: None,
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    // Success:
    let info = mock_info("provider", &[]);
    let res = execute_update_provider(
        deps.as_mut(),
        mock_env(),
        info,
        UpdateProviderMsg {
            id: 0,
            address: None,
            name: Some("new_name".to_string()),
            identity: None,
            website: None,
            security_contact: None,
            details: None,
            information_fee: None,
        },
    )
    .unwrap();

    assert_eq!(0, res.messages.len());

    let providers: Vec<Provider> = th_query(deps.as_ref(), QueryMsg::Providers {});
    assert_eq!(providers[0].name.as_str(), "new_name");
}
