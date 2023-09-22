use crate::helpers::setup;
use cosmwasm_std::{testing::{mock_env, mock_info}, Uint128, Decimal, Timestamp, coins};
use yieldaggregator_bonus::{
    error::ContractError, execute::{update_params::execute_update_params, register_bonus_window::execute_register_bonus_window, stake_vault_share::execute_stake_vault_share}, msgs::{UpdateParamsMsg, RegisterBonusWindowMsg, self, StakeVaultShareMsg},
    query::{params::query_params, bonus_windows::{query_bonus_windows, query_bonus_window}, vault_share_staking::query_vault_share_staking},
};

mod helpers;

#[test]
fn test_stake_vault_share() {
    let mut deps = setup();
    // first, register the bonus window
    let bw_denom = format!("yieldaggregator/vault/{}", "0");
    execute_register_bonus_window(
        deps.as_mut(),
        mock_env(),
        mock_info("authority", &coins(100, bw_denom.clone())),
         RegisterBonusWindowMsg{
                denom: bw_denom.clone(),
                budget_for_all: Uint128::zero(),
                apr_for_winners: vec![Decimal::zero()],
                start_at: Timestamp::default(),
                end_at: Timestamp::default().plus_seconds(1),
            },
        ).unwrap();

    // Error due to invalid denom
    {   
        let info = mock_info(
            "anyone",
            &coins(100, "invalid_denom".to_string())
        );

        let stake_msg = StakeVaultShareMsg {
            bonus_window_id: 0,
            vault_id: 0,
        };
        let err = execute_stake_vault_share(
            deps.as_mut(),
            mock_env(),
            info,
            stake_msg
        ).unwrap_err();

        assert_eq!(ContractError::NoAllowedToken {  }, err);
    }

    // Error due to invalid time window
    {
        let info = mock_info("anyone", &coins(100, bw_denom.clone()));
        let stake_msg = StakeVaultShareMsg {
            bonus_window_id: 0,
            vault_id: 0,
        };
        let err = execute_stake_vault_share(
            deps.as_mut(),
            mock_env(),
            info,
            stake_msg
        ).unwrap_err();

        assert_eq!(ContractError::InvalidBonusWindowPeriod {  }, err);
    }

    // Success
    {
        let bw_denom = format!("yieldaggregator/vault/{}", "0");
        execute_register_bonus_window(
            deps.as_mut(),
            mock_env(),
            mock_info("authority", &coins(100, bw_denom.clone())),
             RegisterBonusWindowMsg{
                    denom: bw_denom.clone(),
                    budget_for_all: Uint128::zero(),
                    apr_for_winners: vec![Decimal::zero()],
                    start_at: Timestamp::default(),
                    end_at: mock_env().block.time.plus_seconds(1000),
                },
            ).unwrap();

        let info = mock_info("anyone", &coins(100, bw_denom.clone()));
        let stake_msg = StakeVaultShareMsg {
            bonus_window_id: 1,
            vault_id: 0,
        };
        let res = execute_stake_vault_share(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            stake_msg
        ).unwrap();

        assert_eq!(0, res.messages.len());

        // meaning test of the vault share query
        let vault_share = query_vault_share_staking(deps.as_ref(), 0, info.clone().sender.into_string()).unwrap();
        assert_eq!(vault_share.vault_share, Uint128::new(100));
    }

    // Error due to already staked
    {
        let info = mock_info("anyone", &coins(100, bw_denom.clone()));
        let stake_msg = StakeVaultShareMsg {
            bonus_window_id: 1,
            vault_id: 0,
        };
        let err = execute_stake_vault_share(
            deps.as_mut(),
            mock_env(),
            info,
            stake_msg
        ).unwrap_err();

        assert_eq!(err, ContractError::AlreadyStaked {  });
    }
}
