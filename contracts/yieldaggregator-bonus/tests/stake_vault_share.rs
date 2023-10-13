use crate::helpers::setup;
use cosmwasm_std::{testing::{mock_env, mock_info}, Uint128, Timestamp, coins, Decimal};
use yieldaggregator_bonus::{
    error::ContractError, execute::{register_bonus_window::execute_register_bonus_window, stake_vault_share::execute_stake_vault_share}, msgs::{RegisterBonusWindowMsg, StakeVaultShareMsg},
    query::{bonus_windows::{query_bonus_windows, query_bonus_window}, vault_share_staking::{query_vault_share_staking, query_total_staking_info, query_total_staking_info_for_a_vault}}, state::TOTAL_STAKING_INFO,
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
                budget_for_all: Uint128::new(100),
                reward_for_winners: vec![Uint128::zero()],
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
                    budget_for_all: Uint128::new(100),
                    reward_for_winners: vec![Uint128::zero()],
                    start_at: Timestamp::default(),
                    end_at: mock_env().block.time.plus_hours(72),
                },
            ).unwrap();

        let info = mock_info("anyone", &coins(10000000000000000, bw_denom.clone()));
        let stake_msg = StakeVaultShareMsg {
            bonus_window_id: 1,
            vault_id: 0,
        };
        let res = execute_stake_vault_share(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            stake_msg.clone()
        ).unwrap();
        assert_eq!(0, res.messages.len());

        // meaning test of the vault share query
        let vault_share = query_vault_share_staking(deps.as_ref(), stake_msg.bonus_window_id, stake_msg.vault_id, info.clone().sender.into_string()).unwrap();
        assert_eq!(vault_share.vault_share, Uint128::new(10000000000000000));

        let mut staking_info = query_total_staking_info_for_a_vault(deps.as_ref(), stake_msg.bonus_window_id, stake_msg.vault_id).unwrap();
        assert_eq!(staking_info.total_staked_amount, Uint128::new(10000000000000000));
        let mut expected_staking_power_index = info.funds[0].amount.checked_mul(Uint128::new(72)).unwrap();
        assert_eq!(staking_info.total_staking_power_index, Decimal::from_ratio(expected_staking_power_index, Uint128::one()));

        let total_staking_power_index = query_total_staking_info(deps.as_ref(), stake_msg.bonus_window_id).unwrap();
        assert_eq!(total_staking_power_index.len(), 1);

        // stake again
        let info = mock_info("anyone2", &coins(1, bw_denom.clone()));
        let stake_msg = StakeVaultShareMsg {
            bonus_window_id: 1,
            vault_id: 0,
        };
        execute_stake_vault_share(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            stake_msg.clone()
        ).unwrap();

        staking_info = query_total_staking_info_for_a_vault(deps.as_ref(), stake_msg.bonus_window_id, stake_msg.vault_id).unwrap();
        assert_eq!(staking_info.total_staked_amount, Uint128::new(10000000000000001));
        expected_staking_power_index = expected_staking_power_index.checked_add(info.funds[0].amount.checked_mul(Uint128::new(72)).unwrap()).unwrap();
        assert_eq!(staking_info.total_staking_power_index, Decimal::from_ratio(expected_staking_power_index, Uint128::one()));
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
