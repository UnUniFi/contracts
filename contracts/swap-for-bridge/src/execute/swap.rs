use crate::error::ContractError;
use crate::msgs::SwapMsg;
use crate::state::PARAMS;
use cosmwasm_std::BankMsg;
use cosmwasm_std::Coin;
use cosmwasm_std::CosmosMsg;
use cosmwasm_std::Decimal;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw_utils::one_coin;

#[cfg(not(feature = "library"))]
pub fn execute_swap(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: SwapMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();
    let config = PARAMS.load(deps.storage)?;

    let coin = one_coin(&info)?;

    if !config.denoms_same_origin.contains(&coin.denom) || 
        !config.denoms_same_origin.contains(&msg.output_denom)  {
        return Err(ContractError::InvalidDenom);
    }

    let fee = Decimal::from_atomics(coin.amount, 0)?
        .checked_mul(config.fee_rate)?
        .to_uint_floor();
    let lp_fee = Decimal::from_atomics(coin.amount, 0)?
        .checked_mul(config.lp_fee_rate)?
        .to_uint_floor();
    let fee_subtracted = coin.amount.checked_sub(fee)?.checked_sub(lp_fee)?;

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
        to_address: config.fee_collector.to_string(),
        amount: vec![Coin {
            denom: coin.denom,
            amount: fee,
        }],
    }));

    response = response.add_attribute("action", "swap");

    Ok(response)
}
