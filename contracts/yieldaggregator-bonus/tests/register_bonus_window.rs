use crate::helpers::setup;
use cosmwasm_std::{testing::{mock_env, mock_info}, Uint128, Decimal, Timestamp, Coin, coins};
use yieldaggregator_bonus::{
    error::ContractError, execute::{register_bonus_window::execute_register_bonus_window}, msgs::{UpdateParamsMsg, RegisterBonusWindowMsg, self},
    query::{params::query_params, bonus_windows::{query_bonus_windows, query_bonus_window}},
};

mod helpers;

#[test]
fn test_register_bonus_window() {
    let mut deps = setup();

    // Error due to the permission
    {
        let invalid_info = mock_info("anyone", &[]);
        let msg = RegisterBonusWindowMsg {
            denom: "tst".to_string(),
            budget_for_all: Uint128::zero(),
            apr_for_winners: vec![Decimal::zero()],
            start_at: Timestamp::default(),
            end_at: Timestamp::default(),
        };
        let err = execute_register_bonus_window(
            deps.as_mut(),
            mock_env(),
            invalid_info,
            msg,
        ).unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});
    }

    // Error due to the insufficient budget
    {
        let info = mock_info("authority", &coins(10, "tst"));
        let msg = RegisterBonusWindowMsg {
            denom: "tst".to_string(),
            budget_for_all: Uint128::new(100),
            apr_for_winners: vec![Decimal::zero()],
            start_at: Timestamp::default(),
            end_at: Timestamp::default(),
        };
        let err = execute_register_bonus_window(
            deps.as_mut(),
            mock_env(),
            info,
            msg,
        ).unwrap_err();
        assert_eq!(err, ContractError::InsufficientBudget {});
    }

    // Success
    {
        let info = mock_info("authority", &coins(100, "tst"));
        let msg = RegisterBonusWindowMsg {
            denom: "tst".to_string(),
            budget_for_all: Uint128::new(100),
            apr_for_winners: vec![Decimal::zero()],
            start_at: Timestamp::default(),
            end_at: Timestamp::default(),
        };
        let res = execute_register_bonus_window(
            deps.as_mut(),
            mock_env(),
            info,
            msg,
        ).unwrap();

        assert_eq!(0, res.messages.len());

        // meaninng test of the windows query
        let bonus_windows = query_bonus_windows(deps.as_ref()).unwrap();
        assert_eq!(1, bonus_windows.len());

        // meaning test of window query
        let bonus_window = query_bonus_window(deps.as_ref(), 0).unwrap();
        assert_eq!(bonus_window.id, 0);
    }   

    // Success case to check id sequencing
    {
        let info = mock_info("authority", &coins(100, "tst"));
        let msg = RegisterBonusWindowMsg {
            denom: "tst".to_string(),
            budget_for_all: Uint128::zero(),
            apr_for_winners: vec![Decimal::zero()],
            start_at: Timestamp::default(),
            end_at: Timestamp::default(),
        };
        let res = execute_register_bonus_window(
            deps.as_mut(),
            mock_env(),
            info,
            msg,
        ).unwrap();

        assert_eq!(0, res.messages.len());

        let bonus_window = query_bonus_window(deps.as_ref(), 1).unwrap();
        assert_eq!(bonus_window.id, 1);
    }

}
