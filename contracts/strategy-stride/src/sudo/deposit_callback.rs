use crate::error::ContractError;
use crate::state::{DepositInfo, DEPOSITS, STATE};
use cosmwasm_std::{DepsMut, Env, Response, StdResult, Uint128};
use std::str::FromStr;
use ununifi_binding::v1::binding::UnunifiMsg;

pub fn sudo_deposit_callback(
    deps: DepsMut,
    _env: Env,
    denom: String,
    amount: String,
    sender: String,
    receiver: String,
    success: bool,
) -> Result<Response<UnunifiMsg>, ContractError> {
    deps.api
        .debug(format!("WASMDEBUG: sudo_deposit_callback received",).as_str());
    if !success {
        return Err(ContractError::Payment(cw_utils::PaymentError::NoFunds {}));
    }

    let sender = deps.api.addr_validate(sender.as_str())?;
    let mut state = STATE.load(deps.storage)?;
    let amount = Uint128::from_str(amount.as_str())?;
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

    let res = Response::new()
        .add_attribute("action", "vault_deposit_stake".to_string())
        .add_attribute("denom", denom.to_string())
        .add_attribute("amount", amount.to_string())
        .add_attribute("sender", sender.to_string())
        .add_attribute("receiver", receiver.to_string());
    return Ok(res);
}
