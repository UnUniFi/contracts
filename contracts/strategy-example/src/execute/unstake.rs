use crate::error::ContractError;
use crate::state::{DEPOSITS, PARAMS};
use crate::types::DepositInfo;
use cosmwasm_std::coins;
use cosmwasm_std::{Addr, BankMsg, CosmosMsg, DepsMut, Response, StdResult, Uint128};

#[cfg(not(feature = "library"))]
pub fn execute_unstake(
    deps: DepsMut,
    amount: Uint128,
    sender: Addr,
) -> Result<Response, ContractError> {
    let mut params = PARAMS.load(deps.storage)?;
    let redemption_rate_multiplier = Uint128::from(1000000u128);
    DEPOSITS.update(
        deps.storage,
        sender.to_string(),
        |deposit: Option<DepositInfo>| -> StdResult<_> {
            if let Some(unwrapped) = deposit {
                let unstake_amount = amount * redemption_rate_multiplier / params.redemption_rate;
                return Ok(DepositInfo {
                    sender: sender.clone(),
                    amount: unwrapped.amount.checked_sub(unstake_amount)?,
                });
            }
            Ok(DepositInfo {
                sender: sender.clone(),
                amount: amount,
            })
        },
    )?;

    params.total_deposit = params
        .total_deposit
        .checked_sub(amount)
        .unwrap_or(Uint128::from(0u128));
    PARAMS.save(deps.storage, &params)?;
    let bank_send_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: sender.to_string(),
        amount: coins(amount.u128(), &params.deposit_denom),
    });
    let rsp = Response::new()
        .add_message(bank_send_msg)
        .add_attribute("action", "unstake")
        .add_attribute("sender", sender)
        .add_attribute("amount", amount);
    Ok(rsp)
}
