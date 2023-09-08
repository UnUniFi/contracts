use crate::error::ContractError;
use crate::msgs::StakeVaultShareMsg;
use crate::state::{BONUS_WINDOWS, VAULT_SHARE_STAKINGS};
use crate::types::VaultShareStaking;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw_utils::one_coin;

#[cfg(not(feature = "library"))]
pub fn execute_stake_vault_share(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: StakeVaultShareMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();
    let coin = one_coin(&info)?;

    let denom = format!("yieldaggregator/vault/{}", msg.vault_id);
    if denom != coin.denom {
        return Err(ContractError::NoAllowedToken {});
    }

    if VAULT_SHARE_STAKINGS.has(deps.storage, (msg.vault_id, info.sender.clone())) {
        return Err(ContractError::AlreadyStaked {});
    }

    let bonus_window = BONUS_WINDOWS.load(deps.storage, msg.bonus_window_id)?;

    if env.block.time < bonus_window.start_at || bonus_window.end_at < env.block.time {
        return Err(ContractError::InvalidBonusWindowPeriod {});
    }

    let staking = VaultShareStaking {
        vault_id: msg.vault_id,
        address: info.sender.clone(),
        vault_share: coin.amount,
        start_at: env.block.time,
    };
    VAULT_SHARE_STAKINGS.save(deps.storage, (msg.vault_id, info.sender), &staking)?;

    response = response.add_attribute("action", "stake_vault_share");

    Ok(response)
}
