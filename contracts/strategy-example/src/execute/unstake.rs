use crate::error::ContractError;
use crate::state::{DepositInfo, CONFIG, DEPOSITS};
use cosmwasm_std::{Addr, BankMsg, CosmosMsg, DepsMut, Response, StdResult, Uint128};

#[cfg(not(feature = "library"))]
pub fn execute_unstake(
    deps: DepsMut,
    amount: Uint128,
    sender: Addr,
) -> Result<Response, ContractError> {
    use cosmwasm_std::coins;

    let mut config = CONFIG.load(deps.storage)?;
    let redemption_rate_multiplier = Uint128::from(1000000u128);
    DEPOSITS.update(
        deps.storage,
        sender.to_string(),
        |deposit: Option<DepositInfo>| -> StdResult<_> {
            if let Some(unwrapped) = deposit {
                let unstake_amount = amount * redemption_rate_multiplier / config.redemption_rate;
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

    config.total_deposit -= amount;
    CONFIG.save(deps.storage, &config)?;
    let bank_send_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: sender.to_string(),
        amount: coins(amount.u128(), &config.deposit_denom),
    });
    let rsp = Response::new()
        .add_message(bank_send_msg)
        .add_attribute("action", "unstake")
        .add_attribute("sender", sender)
        .add_attribute("amount", amount);
    Ok(rsp)
}
