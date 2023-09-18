use crate::helpers::setup;
use cosmwasm_std::{
    testing::{mock_env, mock_info},
    Uint128,
};
use strategy_custodian::{
    error::ContractError, execute::report_profit::execute_report_profit, msgs::ReportProfitMsg, query::share_and_deposit::query_total_deposit,
};

mod helpers;

#[test]
fn test_report_profit() {
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

    let total_deposit = query_total_deposit(deps.as_ref()).unwrap();
    assert_eq!(Uint128::new(100), total_deposit.total_deopsit);
}
