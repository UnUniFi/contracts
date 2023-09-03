use crate::helpers::setup;
use cosmwasm_std::testing::{mock_env, mock_info};
use helpers::th_query;
use strategy::v1::msgs::StakeMsg;
use strategy_custodian::execute::stake::execute_stake;

mod helpers;

#[test]
fn test_stake() {
    let mut deps = setup();

    let info = mock_info("authority", &[]);

    // Error: because of the permission
    let invalid_info = mock_info("anyone", &[]);
    let err = execute_stake(
        deps.as_mut(),
        mock_env(),
        invalid_info,
        StakeMsg {
            provider_id: 0,
            customer: "customer".to_string(),
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    // Success:
    let info = mock_info("provider", &[]);
    let res = execute_stake(
        deps.as_mut(),
        mock_env(),
        info,
        CreateVerificationMsg {
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
    assert_eq!(1, verifications.len());
}