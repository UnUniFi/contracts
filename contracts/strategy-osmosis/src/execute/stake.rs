use crate::error::ContractError;
use crate::state::{DepositInfo, CONFIG, DEPOSITS, STAKE_RATE_MULTIPLIER, STATE};
use cosmwasm_std::{Addr, Coin, DepsMut, Env, Response, StdResult};
use ununifi_binding::v0::binding::UnunifiMsg;

pub fn execute_stake(
    deps: DepsMut,
    _env: Env,
    coin: Coin,
    sender: Addr,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if config.controller_deposit_denom != coin.denom {
        return Err(ContractError::NoAllowedToken {});
    }
    let mut state = STATE.load(deps.storage)?;
    let amount = coin.amount;
    let share_amount = amount * STAKE_RATE_MULTIPLIER / state.redemption_rate;
    state.total_shares += share_amount;
    DEPOSITS.update(
        deps.storage,
        sender.to_string(),
        |deposit: Option<DepositInfo>| -> StdResult<_> {
            if let Some(unwrapped) = deposit {
                return Ok(DepositInfo {
                    sender: sender.clone(),
                    amount: unwrapped.amount.checked_add(share_amount)?,
                });
            }
            Ok(DepositInfo {
                sender: sender.clone(),
                amount: share_amount,
            })
        },
    )?;
    state.total_deposit += amount;
    STATE.save(deps.storage, &state)?;

    let rsp = Response::default()
        .add_attribute("action", "stake")
        .add_attribute("sender", sender)
        .add_attribute("amount", amount)
        .add_attribute("share_amount", share_amount);
    Ok(rsp)
}
