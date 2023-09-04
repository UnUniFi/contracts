use crate::helpers::setup;
use cosmwasm_std::{
    testing::{mock_env, mock_info},
    Uint128,
};
use helpers::th_query;
use strategy_custodian::{
    error::ContractError, execute::report_profit::execute_report_profit, msgs::ReportProfitMsg,
};

mod helpers;

#[test]
fn test_stake() {
    let mut deps = setup();

    // Error: because of the permission
    let invalid_info = mock_info("anyone", &[]);
    let err = execute_report_profit(
        deps.as_mut(),
        mock_env(),
        invalid_info,
        ReportProfitMsg {
            profit: Uint128::new(100),
            is_positive: true,
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    // Success:
    let info = mock_info("admin", &[]);
    let res = execute_report_profit(
        deps.as_mut(),
        mock_env(),
        info,
        ReportProfitMsg {
            profit: Uint128::new(100),
            is_positive: true,
        },
    )
    .unwrap();

    assert_eq!(0, res.messages.len());
}
