use crate::error::ContractError;
use crate::state::{BONUS_WINDOWS, TOTAL_STAKING_INFO, VOTED_VAULTS, VaultId, PARAMS, VAULT_SHARE_STAKINGS, TotalStakedAmount, TotalStakingPowerIndex};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Decimal, Uint128, Storage, Addr, StdResult, Order, CosmosMsg, Coins, Coin, BankMsg};
use crate::msgs::{DistributionMsg, PriceData};
use crate::types::VotedVault;
use std::collections::HashMap;

#[cfg(not(feature = "library"))]
pub fn execute_distribution(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: DistributionMsg,
) -> Result<Response, ContractError> {
    use std::vec;

    let mut res = Response::new();
    let params = PARAMS.load(deps.storage)?;

    // Permission check
    if info.sender != params.authority {
        return Err(ContractError::Unauthorized {});
    }

    let bonus_window = BONUS_WINDOWS.load(deps.storage, msg.bonus_window_id)?;
    // Check if the bonus window is already ended
    if env.block.time < bonus_window.end_at {
        return Err(ContractError::BonusWindowNotEndedYet {});
    }

    // Get vault ids out of kvstore instead of price data because the price data may lack some part of vault ids
    let mut voted_vault_ids = Vec::new();
    let total_staking_info_for_all = TOTAL_STAKING_INFO
        .prefix(msg.bonus_window_id)
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item_result| -> StdResult<_> {
            let (vault_id, vault_staking_info) = item_result?;
            voted_vault_ids.push(vault_id.to_owned());
            Ok( (vault_id, (vault_staking_info.0, vault_staking_info.1)) )
        })
        .collect::<StdResult<HashMap<VaultId, (TotalStakedAmount, TotalStakingPowerIndex)>>>()?;
    
    // Check the number of the vaults in price data and extracted vault ids
    if voted_vault_ids.len() != msg.price_data.len() {
        return Err(ContractError::InvalidVaultShareTokenPriceData {});
    }

    let normalized_price_data = normalize_price_data(msg.price_data.clone())?;

    let mut voted_vault = VOTED_VAULTS
        .prefix(msg.bonus_window_id)
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item_result| -> StdResult<_> {
            let (vault_id, total_voted_amount) = item_result?;
            Ok( VotedVault {
                vault_id: vault_id,
                voted_amount: total_voted_amount
            })
        })
        .collect::<StdResult<Vec<VotedVault>>>()?;
    // Sort the staking power index in descending order by the voted amount
    // NOTE: If the voted amounts are the same, the order follows the order of the vault id (if it's all num)
    voted_vault.sort_by(|a, b| b.voted_amount.cmp(&a.voted_amount));
    
    let mut count = 0;
    let voting_winners = voted_vault
        .iter()
        .take(bonus_window.reward_for_winners.len())
        .map(|item| -> StdResult<_> {
            let reward = bonus_window.reward_for_winners[count];
            count += 1;
            Ok((item.vault_id.clone(), reward))
        })
        .collect::<StdResult<HashMap<VaultId, Uint128>>>()?;

    let total_voted_amount = voted_vault
        .iter()
        .fold(Uint128::zero(), |acc, item| acc + item.voted_amount);
    
    // Calculation for Rewards for winners:
    // Calculate the ratio of each staker's staking power index to the total staking power index
    // Multiply the ratio by the reward_for_winners
    let total_staked_amount_in_same_unit = calculate_total_staked_amount_in_same_unit(
        &voted_vault_ids,
        normalized_price_data.clone(),
        total_staking_info_for_all.clone(),
    )?;

    let reward_for_stakers = calculate_total_rewards_for_each_staker(
        deps.storage, 
        msg.bonus_window_id,
        &voted_vault_ids, 
        voting_winners, 
        bonus_window.budget_for_all.checked_add(total_voted_amount)?,
        normalized_price_data,
        total_staked_amount_in_same_unit,
        total_staking_info_for_all,
    )?;
    
    let send_msgs = create_send_msg_for_rewards(
        &reward_for_stakers,
        bonus_window.denom,
    )?;
    
    res = res.add_messages(send_msgs);

    // BonusWindow isn't deleted here because it's needed for the determination of the window id
    // BUT, we can set the other way to construct the window id
    // SO, after the implementation of that, we can delete the BonusWindow here
    delete_unnecessary_data(deps.storage, msg.bonus_window_id, voted_vault_ids, reward_for_stakers.keys().collect())?;

    res = res.add_attributes(vec![
        attr("action", "distribution"),
        attr("total_voted_amount", total_voted_amount),
        attr("vote_winners", format!("{:?}", voting_winners)),
        attr("total_staking_info", format!("{:?}", total_staking_info_for_all)),
    ]);
    
    Ok(res)
}

// Calculate the total staked amount in the same unit (e.g. USD)
pub fn calculate_total_staked_amount_in_same_unit(
    vault_ids: &Vec<VaultId>,
    price_data: HashMap<VaultId, Decimal>,
    total_staking_info_for_all: HashMap<VaultId, (TotalStakedAmount, TotalStakingPowerIndex)>,
) -> StdResult<Decimal> {
    let mut total_staked_amount_in_same_unit = Decimal::zero();

    for vault_id in vault_ids {
        let vault_share_token_value = price_data
            .iter()
            .find(|item| item.0 == vault_id)
            .unwrap()
            .1;
        
        // NOTE:
        // We might have to consider Multiplication overflow error here
        // it emits the error even if it's less the max size of 128 bit
        total_staked_amount_in_same_unit = total_staked_amount_in_same_unit
            .checked_add(
                Decimal::from_ratio(
                    total_staking_info_for_all.get(&vault_id).unwrap().0,
                    Uint128::one()
                )
                    .checked_mul(*vault_share_token_value)
                    .unwrap()
            )?;
    }
    Ok(total_staked_amount_in_same_unit)
}

// create the conprehensive sequence of price ratio for each vault
pub fn normalize_price_data(
    price_data: Vec<PriceData>,
) -> StdResult<HashMap<VaultId, Decimal>> {
    let sum_of_vault_share_token_valut = price_data
    .iter()
    .fold(Decimal::zero(), |acc, data| acc + data.price);

    let normalized_price_data = price_data
        .iter()
        .map(|item| -> StdResult<_> {
            let vault_share_token_value = item.price
                .checked_div(sum_of_vault_share_token_valut)
                .unwrap();
            Ok((item.vault_id, vault_share_token_value))
        })
        .collect::<StdResult<HashMap<VaultId, Decimal>>>()?;
    
    Ok(normalized_price_data)
}

pub fn calculate_total_rewards_for_each_staker(
    store: &dyn Storage,
    bonus_window_id: u64,
    vault_ids: &Vec<VaultId>,
    voting_winners: HashMap<VaultId, Uint128>,
    total_potential_reward: Uint128,
    price_data: HashMap<VaultId, Decimal>,
    normalized_total_staked_amount_in_same_unit: Decimal,
    total_staking_info_for_all: HashMap<VaultId, (TotalStakedAmount, TotalStakingPowerIndex)>,
) -> StdResult<HashMap<Addr, Coins>> {
    let mut rewards = HashMap::<Addr, Coins>::new();
    
    for vault_id in vault_ids {
        let price = price_data
            .iter()
            .find(|item| item.0 == vault_id)
            .unwrap()
            .1;

        let mut rewards_in_vault_id = VAULT_SHARE_STAKINGS
            .prefix((bonus_window_id, vault_id.to_owned()))
            .range(store, None, None, Order::Ascending)
            .map(|item| -> StdResult<_> {
                let (addr, staking) = item?;

                let proportion_of_staking = Decimal::from_ratio(staking.vault_share, Uint128::one())
                    .checked_mul(*price)?
                    .checked_div(normalized_total_staked_amount_in_same_unit)
                    .unwrap();
                let mut total_reward = Decimal::to_uint_floor(Decimal::from_ratio(total_potential_reward, Uint128::one())
                    .checked_mul(proportion_of_staking)?);

                if voting_winners.keys().to_owned().collect::<Vec<&VaultId>>().contains(&&vault_id) {
                    let all_reward_in_votintg= voting_winners.get(&vault_id).unwrap();
                    let proportional_reward_in_voting = staking.staking_power_index
                        .checked_div(total_staking_info_for_all.get(&vault_id).unwrap().1)
                        .unwrap()
                        .checked_mul(Decimal::from_ratio(*all_reward_in_votintg, Uint128::one()))?;
                        
                    total_reward = total_reward.checked_add(Decimal::to_uint_floor(proportional_reward_in_voting))?;
                }

                let mut coins = Coins::default();
                let vault_share_denom = format!("yieldaggregator/vault/{}", vault_id);
                coins.add(Coin::new(total_reward.into(), "".to_string()))?;
                coins.add(Coin::new(staking.vault_share.into(), vault_share_denom))?;
        
                Ok((addr, coins))
            })
            .collect::<StdResult<HashMap<Addr, Coins>>>()?;

        rewards.extend(rewards_in_vault_id.drain());
    }

    Ok(rewards)
}

// Create SendMsg for each reward receiver
pub fn create_send_msg_for_rewards(
    rewards: &HashMap<Addr, Coins>,
    reward_token_denom: String,
) -> StdResult<Vec<CosmosMsg>> {
    let mut send_msgs = Vec::<CosmosMsg>::new();
    for (addr, coins) in rewards {
        let new_coins = coins.clone().into_vec().iter().map(|coin| {
            if coin.denom == "" {
                let new_coin = Coin::new(coin.amount.into(), reward_token_denom.clone());
                return new_coin
            }
            coin.to_owned()
        }).collect::<Vec<Coin>>();

        let send_msg = CosmosMsg::Bank(BankMsg::Send {
            to_address: addr.to_string(),
            amount: new_coins,
        });
        send_msgs.push(send_msg);
    }
    Ok(send_msgs)
}

pub fn delete_unnecessary_data(
    store: &mut dyn Storage,
    bonus_window_id: u64,
    vault_ids: Vec<VaultId>,
    addrs: Vec<&Addr>,
) -> StdResult<()> {
    for vault_id in vault_ids {
        VOTED_VAULTS.remove(store, (bonus_window_id, vault_id));
        TOTAL_STAKING_INFO.remove(store, (bonus_window_id, vault_id));

        for addr in addrs.clone() {
            VAULT_SHARE_STAKINGS.remove(store, (bonus_window_id, vault_id, addr.to_owned()));
        }
    }

    Ok(())
}
