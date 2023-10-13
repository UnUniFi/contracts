use crate::helpers::setup;
use cosmwasm_std::{testing::{mock_env, mock_info, MockStorage, MockApi, MockQuerier}, Uint128, Timestamp, coins, OwnedDeps, Decimal};
use yieldaggregator_bonus::{
    error::ContractError, execute::{update_params::execute_update_params, register_bonus_window::execute_register_bonus_window, stake_vault_share::execute_stake_vault_share, distribution::execute_distribution, vote::execute_vote}, msgs::{UpdateParamsMsg, RegisterBonusWindowMsg, self, StakeVaultShareMsg, DistributionMsg, PriceData, VoteMsg},
    query::{params::query_params, bonus_windows::{query_bonus_windows, query_bonus_window}, vault_share_staking::query_vault_share_staking}, state::TOTAL_STAKING_INFO,
};

mod helpers;

#[test]
fn test_distributjion() {
    let mut deps = prepare_for_distribution();

    // Error because BonusWindow hasn't ended yet
    {
        // register the bonus window
        let bw_denom = "tst".to_string();
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

        let bonus_window_id = 0;
        let voted_vault_ids = vec![0, 1];

        let env = mock_env();
        let res = execute_distribution(
            deps.as_mut(), 
            env,
            mock_info("authority", &[]),
            DistributionMsg {
                bonus_window_id: bonus_window_id,
                price_data: vec![
                    PriceData {
                        vault_id: voted_vault_ids[0],
                        price: Decimal::one(),
                    },
                    PriceData {
                        vault_id: voted_vault_ids[1],
                        price: Decimal::one(),
                    },
                ],
            },
        ).unwrap_err();
        assert_eq!(res, ContractError::BonusWindowNotEndedYet {  });
    }

    // Error because provided price data are insufficient
    {
        let bonus_window_id = 0;
        let voted_vault_ids = vec![0, 1];

        let mut env = mock_env();
        env.block.time = env.block.time.plus_seconds(2592001);
        let res = execute_distribution(
            deps.as_mut(), 
            env,
            mock_info("authority", &[]),
            DistributionMsg {
                bonus_window_id: bonus_window_id,
                price_data: vec![
                    PriceData {
                        vault_id: voted_vault_ids[0],
                        price: Decimal::one(),
                    },
                ],
            },
        ).unwrap_err();
        assert_eq!(res, ContractError::InvalidVaultShareTokenPriceData {  });
    }

    // Success with the situation below
    // 1. The bonus window is expired
    // 2. The vault 0, 1 are voted for 100, 100
    // 3. The vault 0, 1 are staked
    // 4. "anyone" staked 100 for vault 0 and "anyone2" staked 100 for vault 1
    // 5. price data for each are 1 and 1
    // 6. The budget for all is 100
    // 7. The reward for 1st winners is 100
    // So, the distribution will be followed:
    // "anyone" gets in total 250 = (150 + 100)
    // "anyone2" gets in total 150 = (150 + 0)
    {
        let bonus_window_id = 0;
        let voted_vault_ids = vec![0, 1];

        let mut env = mock_env();
        env.block.time = env.block.time.plus_seconds(2592001);
        let res = execute_distribution(
            deps.as_mut(), 
            env,
            mock_info("authority", &[]),
            DistributionMsg {
                bonus_window_id: bonus_window_id,
                price_data: vec![
                    PriceData {
                        vault_id: voted_vault_ids[0],
                        price: Decimal::one(),
                    },
                    PriceData {
                        vault_id: voted_vault_ids[1],
                        price: Decimal::one(),
                    },
                ],
            },
        );
        println!("{:?}", res);

        assert_eq!(2, res.unwrap().messages.len());

        // Check if the unnecesary data is deleted
        query_vault_share_staking(deps.as_ref(), bonus_window_id, voted_vault_ids[0], "anyone".to_string()).unwrap_err();
        query_vault_share_staking(deps.as_ref(), bonus_window_id, voted_vault_ids[1], "anyone2".to_string()).unwrap_err();

        TOTAL_STAKING_INFO.load(deps.as_ref().storage, (0, 0)).unwrap_err();    
        TOTAL_STAKING_INFO.load(deps.as_ref().storage, (0, 1)).unwrap_err();    
    }
}

fn prepare_for_distribution() ->  OwnedDeps<MockStorage, MockApi, MockQuerier> {
    let mut deps = setup();
    // first, register the bonus window
    let bw_denom = format!("yieldaggregator/vault/{}", "0");
    let reward_token_denom = "reward_token".to_string();
    execute_register_bonus_window(
        deps.as_mut(),
        mock_env(),
        mock_info("authority", &coins(200, reward_token_denom.clone())),
            RegisterBonusWindowMsg{
                denom: reward_token_denom.clone(),
                budget_for_all: Uint128::new(100),
                reward_for_winners: vec![Uint128::new(100)],
                start_at: Timestamp::default(),
                end_at: mock_env().block.time.plus_seconds(2592000),
            },
        ).unwrap();

    let info = mock_info("anyone", &coins(100, bw_denom.clone()));
    let stake_msg = StakeVaultShareMsg {
        bonus_window_id: 0,
        vault_id: 0,
    };
    execute_stake_vault_share(
        deps.as_mut(),
        mock_env(),
        info.clone(),
        stake_msg.clone()
    ).unwrap();

    let info = mock_info("anyone2", &coins(100, format!("yieldaggregator/vault/{}", "1")));
    let stake_msg = StakeVaultShareMsg {
        bonus_window_id: 0,
        vault_id: 1,
    };
    execute_stake_vault_share(
        deps.as_mut(),
        mock_env(),
        info.clone(),
        stake_msg.clone()
    ).unwrap();

    let info = mock_info("anyone",  &coins(100, reward_token_denom.clone()));
    execute_vote(
        deps.as_mut(),
        mock_env(),
        info,
        VoteMsg {
            bonus_window_id: 0,
            vault_id: 0,
        },
    ).unwrap();


    let info = mock_info("anyone2",  &coins(100, reward_token_denom.clone()));
    execute_vote(
        deps.as_mut(),
        mock_env(),
        info,
        VoteMsg {
            bonus_window_id: 0,
            vault_id: 1,
        },
    ).unwrap();

    deps
}

// #[test]

