use crate::error::ContractError;
use crate::msgs::StakeMsg;
use crate::state::{DepositInfo, CONFIG, DEPOSITS, STAKE_RATE_MULTIPLIER, STATE};
use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response, StdResult};
use cw_utils::one_coin;
use ununifi_binding::v1::binding::UnunifiMsg;

pub fn execute_stake(
    deps: DepsMut,
    _: Env,
    info: MessageInfo,
    _: StakeMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let coin: Coin = one_coin(&info).map_err(|err| ContractError::Payment(err))?;
    let sender = info.sender;
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
                amount: amount,
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
