use crate::error::ContractError;

use crate::state::{DepositInfo, CONFIG, DEPOSITS, STAKE_RATE_MULTIPLIER};
use cosmwasm_std::{Addr, Coin, DepsMut, Env, Response, StdResult};
use ununifi_binding::v0::binding::UnunifiMsg;

pub fn execute_stake(
    deps: DepsMut,
    env: Env,
    coin: Coin,
    sender: Addr,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if config.controller_config.deposit_denom != coin.denom {
        return Err(ContractError::NoAllowedToken {});
    }
    let amount = coin.amount;
    DEPOSITS.update(
        deps.storage,
        sender.to_string(),
        |deposit: Option<DepositInfo>| -> StdResult<_> {
            if let Some(unwrapped) = deposit {
                let stake_amount = amount * STAKE_RATE_MULTIPLIER / config.redemption_rate;
                config.total_shares += stake_amount;
                return Ok(DepositInfo {
                    sender: sender.clone(),
                    amount: unwrapped.amount.checked_add(stake_amount)?,
                });
            }
            Ok(DepositInfo {
                sender: sender.clone(),
                amount: amount,
            })
        },
    )?;
    config.total_deposit += amount;
    CONFIG.save(deps.storage, &config)?;

    let rsp = Response::default()
        .add_attribute("action", "stake")
        .add_attribute("sender", sender)
        .add_attribute("amount", amount);
    Ok(rsp)
}
