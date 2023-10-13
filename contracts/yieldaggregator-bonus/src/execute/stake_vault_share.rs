use crate::error::ContractError;
use crate::msgs::StakeVaultShareMsg;
use crate::state::{BONUS_WINDOWS, VAULT_SHARE_STAKINGS, TOTAL_STAKING_INFO};
use crate::types::{VaultShareStaking, BonusWindow};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw_utils::one_coin;
use cosmwasm_std::{StdResult, Uint128};

#[cfg(not(feature = "library"))]
pub fn execute_stake_vault_share(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: StakeVaultShareMsg,
) -> Result<Response, ContractError> {
    use cosmwasm_std::Decimal;

    use crate::state::{TotalStakedAmount, TotalStakingPowerIndex};

    let mut response = Response::new();
    let coin = one_coin(&info)?;

    let denom = format!("yieldaggregator/vault/{}", msg.vault_id);
    if denom != coin.denom {
        return Err(ContractError::NoAllowedToken {});
    }

    let bonus_window: BonusWindow = BONUS_WINDOWS.load(deps.storage, msg.bonus_window_id)?;

    if VAULT_SHARE_STAKINGS.has(deps.storage, (bonus_window.id, msg.vault_id, info.sender.clone())) {
        return Err(ContractError::AlreadyStaked {});
    }

    if env.block.time < bonus_window.start_at || bonus_window.end_at < env.block.time {
        return Err(ContractError::InvalidBonusWindowPeriod {});
    }

    // Staking Power Index follows the formula:
    // staking_power_index = staking_amount * stakikng_period (hours)
    // staking period can derive the start_at time and end_at time of the bonus window
    // because user cannot unstake until the bonus window ends
    let staking_period = bonus_window.end_at.seconds().checked_sub(env.block.time.seconds()).unwrap().checked_div(3600).unwrap();
    let staking_power_index = coin.amount.checked_mul(staking_period.into()).unwrap();

    let staking = VaultShareStaking {
        vault_share: coin.amount,
        // we don't record start_at because staking power index includs the start_at information in the number
        staking_power_index: Decimal::from_ratio(staking_power_index, Uint128::one()),
    };
    VAULT_SHARE_STAKINGS.save(deps.storage, (bonus_window.id, msg.vault_id, info.sender.clone()), &staking.clone())?;

    TOTAL_STAKING_INFO.update(
        deps.storage, 
        (bonus_window.id.clone(), msg.vault_id.clone()),
        |item: Option<(TotalStakedAmount, TotalStakingPowerIndex)>| -> StdResult<_> {
            match item {
                Some(mut item) => {
                    item.0 += coin.amount;
                    item.1 += staking.staking_power_index;
                    Ok(item)
                }
                None => Ok((coin.amount, staking.staking_power_index)),
            }
        }
    )?;

    response = response.add_attribute("action", "stake_vault_share");

    Ok(response)
}
