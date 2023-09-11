use crate::helpers::setup;
use cosmwasm_std::coin;
use cosmwasm_std::testing::{mock_env, mock_info};
use helpers::th_query;
use kyc::error::ContractError;
use kyc::execute::register_provider::execute_register_provider;
use kyc::msgs::{QueryMsg, RegisterProviderMsg};
use kyc::types::Provider;

mod helpers;

#[test]
fn test_register_provider() {
    let mut deps = setup();

    // Error: because of the authority
    let invalid_info = mock_info("anyone", &[]);
    let err = execute_register_provider(
        deps.as_mut(),
        mock_env(),
        invalid_info,
        RegisterProviderMsg {
            address: "anyone".to_string(),
            name: "anyone".to_string(),
            identity: "anyone".to_string(),
            website: "anyone".to_string(),
            security_contact: "anyone".to_string(),
            details: "anyone".to_string(),
            information_fee: coin(0, "denom"),
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    // Success:
    let info = mock_info("authority", &[]);
    let res = execute_register_provider(
        deps.as_mut(),
        mock_env(),
        info,
        RegisterProviderMsg {
            address: "anyone".to_string(),
            name: "anyone".to_string(),
            identity: "anyone".to_string(),
            website: "anyone".to_string(),
            security_contact: "anyone".to_string(),
            details: "anyone".to_string(),
            information_fee: coin(0, "denom"),
        },
    )
    .unwrap();

    assert_eq!(0, res.messages.len());

    let providers: Vec<Provider> = th_query(deps.as_ref(), QueryMsg::Providers {});
    assert_eq!(1, providers.len());
}
