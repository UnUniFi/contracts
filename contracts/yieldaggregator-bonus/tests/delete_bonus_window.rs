use crate::helpers::setup;
use cosmwasm_std::{testing::{mock_env, mock_info}, Uint128, Decimal, Timestamp, coins};
use yieldaggregator_bonus::{
    error::ContractError, execute::{register_bonus_window::execute_register_bonus_window, vote::execute_vote, delete_bonus_window::execute_delete_bonus_window}, msgs::{UpdateParamsMsg, RegisterBonusWindowMsg, VoteMsg, DeleteBonusWindowMsg},
    query::{params::query_params, voted_vaults::query_voted_vaults, bonus_windows::{query_bonus_windows, query_bonus_window}}, 
};

mod helpers;

#[test]
fn test_delete_bonus_window() {
    let mut deps = setup();
    // first, register the bonus window
    let bw_denom = format!("yieldaggregator/vault/{}", "0");
    execute_register_bonus_window(
        deps.as_mut(),
        mock_env(),
        mock_info("authority", &coins(100, bw_denom.clone())),
         RegisterBonusWindowMsg{
                denom: bw_denom.clone(),
                budget_for_all: Uint128::new(100),
                reward_for_winners: vec![Uint128::zero()],
                start_at: Timestamp::default(),
                end_at: Timestamp::default().plus_seconds(1),
            },
        ).unwrap();

    // Error due to invalid sender
    {
        let msg = DeleteBonusWindowMsg {
            bonus_window_id: 0,
        };

        let info = mock_info(
            "anyone",
            &coins(100, bw_denom.clone())
        );
        let err = execute_delete_bonus_window(
            deps.as_mut(),
            mock_env(),
            info,
            msg,
        ).unwrap_err();

        assert_eq!(ContractError::Unauthorized {}, err);
    }
    
    // Success
    {
        let msg = DeleteBonusWindowMsg {
            bonus_window_id: 0,
        };

        let info = mock_info(
            "authority",
            &coins(100, bw_denom.clone())
        );
        let res = execute_delete_bonus_window(
            deps.as_mut(),
            mock_env(),
            info,
            msg,
        ).unwrap();

        assert_eq!(0, res.messages.len());

        // Assume that the bonus window is deleted and returns err 
        let bonus_windows = query_bonus_windows(deps.as_ref());
        assert_eq!(0, bonus_windows.unwrap().len());
        query_bonus_window(deps.as_ref(), 0).unwrap_err();

        let voted_vault = query_voted_vaults(deps.as_ref(), 0).unwrap_err();
        // assert_eq!(voted_vault.len(), 2);
    }
}
