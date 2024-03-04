use crate::error::ContractError;
use crate::state::{DepositInfo, DEPOSITS, PARAMS, STATE};
use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response, StdResult};
use cw_utils::one_coin;
use strategy::v1::msgs::StakeMsg;
use ununifi_binding::v1::binding::UnunifiMsg;

pub fn execute_stake(
    deps: DepsMut,
    _: Env,
    info: MessageInfo,
    _: StakeMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let coin: Coin = one_coin(&info).map_err(|err| ContractError::Payment(err))?;
    let sender = info.sender;
    let params = PARAMS.load(deps.storage)?;
    if params.ls_denom != coin.denom {
        return Err(ContractError::NoAllowedToken {});
    }
    let mut state = STATE.load(deps.storage)?;
    let amount = coin.amount;
    state.total_amount += amount;
    DEPOSITS.update(
        deps.storage,
        sender.to_string(),
        |deposit: Option<DepositInfo>| -> StdResult<_> {
            if let Some(unwrapped) = deposit {
                return Ok(DepositInfo {
                    sender: sender.clone(),
                    amount: unwrapped.amount.checked_add(amount)?,
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
        .add_attribute("amount", amount);
    Ok(rsp)
}
