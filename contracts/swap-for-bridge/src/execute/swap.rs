use crate::error::ContractError;
use crate::msgs::SwapMsg;
use crate::state::PARAMS;
use cosmwasm_std::BankMsg;
use cosmwasm_std::Coin;
use cosmwasm_std::CosmosMsg;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw_utils::one_coin;

#[cfg(not(feature = "library"))]
pub fn execute_swap(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: SwapMsg,
) -> Result<Response, ContractError> {
    use crate::query::fee::query_estimate_fee;

    let mut response = Response::new();
    let params = PARAMS.load(deps.storage)?;

    let coin = one_coin(&info)?;

    if !params.denoms_same_origin.contains(&coin.denom)
        || !params.denoms_same_origin.contains(&msg.output_denom)
    {
        return Err(ContractError::InvalidDenom);
    }

    let fee = query_estimate_fee(deps.as_ref(), coin.amount.clone())?;

    if fee.total_fee > coin.amount {
        return Err(ContractError::InsufficientFundsForMinFee);
    }

    let non_lp_fee = fee.fee;
    let fee_subtracted = coin.amount.checked_sub(fee.total_fee)?;

    response = response.add_message(CosmosMsg::Bank(BankMsg::Send {
        to_address: match msg.recipient {
            Some(recipient) => recipient,
            None => info.sender.to_string(),
        },
        amount: vec![Coin {
            denom: msg.output_denom,
            amount: fee_subtracted,
        }],
    }));

    response = response.add_message(CosmosMsg::Bank(BankMsg::Send {
        to_address: params.fee_collector.to_string(),
        amount: vec![Coin {
            denom: coin.denom,
            amount: non_lp_fee,
        }],
    }));

    response = response.add_attribute("action", "swap");

    Ok(response)
}
