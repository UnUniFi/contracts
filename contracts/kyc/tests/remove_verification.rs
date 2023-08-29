use crate::helpers::setup;
use cosmwasm_std::testing::{mock_env, mock_info};
use helpers::th_query;
use kyc::error::ContractError;
use kyc::execute::create_verification::execute_create_verification;
use kyc::execute::register_provider::execute_register_provider;
use kyc::execute::remove_verification::execute_remove_verification;
use kyc::msgs::{CreateVerificationMsg, QueryMsg, RegisterProviderMsg, RemoveVerificationMsg};
use kyc::types::Verification;

mod helpers;

#[test]
fn test_remove_verification() {
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
        },
    )
    .unwrap();

    let info = mock_info("provider", &[]);
    execute_create_verification(
        deps.as_mut(),
        mock_env(),
        info,
        CreateVerificationMsg {
            provider_id: 0,
            customer: "customer".to_string(),
        },
    )
    .unwrap();

    // Error: because of the permission
    let invalid_info = mock_info("anyone", &[]);
    let err = execute_remove_verification(
        deps.as_mut(),
        mock_env(),
        invalid_info,
        RemoveVerificationMsg {
            provider_id: 0,
            customer: "customer".to_string(),
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    let verifications: Vec<Verification> = th_query(
        deps.as_ref(),
        QueryMsg::Verifications {
            address: "customer".to_string(),
        },
    );
    assert_eq!(1, verifications.len());

    // Success:
    let info = mock_info("provider", &[]);
    let res = execute_remove_verification(
        deps.as_mut(),
        mock_env(),
        info,
        RemoveVerificationMsg {
            provider_id: 0,
            customer: "customer".to_string(),
        },
    )
    .unwrap();

    assert_eq!(0, res.messages.len());

    let verifications: Vec<Verification> = th_query(
        deps.as_ref(),
        QueryMsg::Verifications {
            address: "customer".to_string(),
        },
    );
    assert_eq!(0, verifications.len());
}
