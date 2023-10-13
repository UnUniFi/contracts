use crate::helpers::setup;
use cosmwasm_std::{testing::{mock_env, mock_info}, Uint128, Decimal, Timestamp, coins};
use yieldaggregator_bonus::{
    error::ContractError, execute::{update_params::execute_update_params, register_bonus_window::execute_register_bonus_window, vote::execute_vote}, msgs::{UpdateParamsMsg, RegisterBonusWindowMsg, VoteMsg},
    query::{params::query_params, voted_vaults::query_voted_vaults},
};

mod helpers;

#[test]
fn test_vote() {
    let mut deps = setup();

    // First, register the bonus window
    execute_register_bonus_window(
        deps.as_mut(),
        mock_env(),
        mock_info("authority", &coins(100, "test")),
            RegisterBonusWindowMsg{
                denom: "test".to_string(),
                budget_for_all: Uint128::new(100),
                reward_for_winners: vec![Uint128::zero()],
                start_at: Timestamp::default(),
                end_at: mock_env().block.time.plus_seconds(1000),
            },
        ).unwrap();

    // Error: because of the invalid token
    {
        let invalid_info = mock_info("anyone",  &coins(100, "invalid"));
        let err = execute_vote(
            deps.as_mut(),
            mock_env(),
            invalid_info,
            VoteMsg {
                bonus_window_id: 0,
                vault_id: 0,
            },
        ).unwrap_err();

        assert_eq!(err, ContractError::NoAllowedToken {});
    }

    // Error: because of the invalid bonus time window 
    {
        execute_register_bonus_window(
            deps.as_mut(),
            mock_env(),
            mock_info("authority", &coins(100, "test")),
                RegisterBonusWindowMsg{
                    denom: "test".to_string(),
                    budget_for_all: Uint128::new(100),
                    reward_for_winners: vec![Uint128::zero()],
                    start_at: Timestamp::default(),
                    end_at: Timestamp::default().plus_seconds(1)
                },
            ).unwrap();

        let info = mock_info("anyone",  &coins(100, "test"));
        let err = execute_vote(
            deps.as_mut(),
            mock_env(),
            info,
            VoteMsg {
                bonus_window_id: 1,
                vault_id: 0,
            },
        ).unwrap_err();

        assert_eq!(err, ContractError::InvalidBonusWindowPeriod {  });
    }

    // Success for the first vote
    {
        let info = mock_info("anyone",  &coins(100, "test"));
        let res = execute_vote(
            deps.as_mut(),
            mock_env(),
            info,
            VoteMsg {
                bonus_window_id: 0,
                vault_id: 0,
            },
        ).unwrap();

        assert_eq!(0, res.messages.len());

        let voted_vault = query_voted_vaults(deps.as_ref(), 0).unwrap();
        assert_eq!(voted_vault.len(), 1);
        assert_eq!(voted_vault[0].voted_amount, Uint128::new(100));
    }

    // Success for the second vote
    {
        let info = mock_info("anyone",  &coins(100, "test"));
        let res = execute_vote(
            deps.as_mut(),
            mock_env(),
            info,
            VoteMsg {
                bonus_window_id: 0,
                vault_id: 2,
            },
        ).unwrap();

        assert_eq!(0, res.messages.len());

        let voted_vault = query_voted_vaults(deps.as_ref(), 0).unwrap();
        assert_eq!(voted_vault.len(), 2);
        assert_eq!(voted_vault[1].vault_id, 2);
        assert_eq!(voted_vault[1].voted_amount, Uint128::new(100));
    }
}
