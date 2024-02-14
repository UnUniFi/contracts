use crate::error::{ContractError, NoDeposit};
use crate::state::{DepositInfo, DEPOSITS, PARAMS, STATE};

use cosmwasm_std::{
    coins, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use strategy::v1::msgs::UnstakeMsg;
use ununifi_binding::v1::binding::UnunifiMsg;

pub fn execute_unstake(
    deps: DepsMut,
    _: Env,
    info: MessageInfo,
    msg: UnstakeMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let amount = msg.share_amount;
    let sender = info.sender;
    let recipient = msg.recipient;
    let mut state = STATE.load(deps.storage)?;
    DEPOSITS.update(
        deps.storage,
        sender.to_string(),
        |deposit: Option<DepositInfo>| -> StdResult<_> {
            if let Some(unwrapped) = deposit {
                return Ok(DepositInfo {
                    sender: sender.clone(),
                    amount: unwrapped.amount.checked_sub(amount)?,
                });
            }
            Err(NoDeposit {}.into())
        },
    )?;

    let mut recipient_addr = sender.to_owned();
    if let Some(recipient_str) = recipient {
        recipient_addr = deps.api.addr_validate(recipient_str.as_str())?;
    }

    state.total_amount = state
        .total_amount
        .checked_sub(amount)
        .unwrap_or(Uint128::from(0u128));

    state.total_withdrawn += amount;
    STATE.save(deps.storage, &state)?;

    let params = PARAMS.load(deps.storage)?;
    let bank_send_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: recipient_addr.to_string(),
        amount: coins(amount.u128(), &params.ls_denom),
    });

    let rsp = Response::new()
        .add_attribute("action", "unstake")
        .add_attribute("sender", sender.to_string())
        .add_attribute("recipient", recipient_addr.to_string())
        .add_attribute("amount", amount)
        .add_message(bank_send_msg);
    Ok(rsp)
}
