use crate::error::ContractError;
use crate::msgs::SwapMsg;
use crate::state::CONFIG;
use cosmwasm_std::BankMsg;
use cosmwasm_std::Coin;
use cosmwasm_std::CosmosMsg;
use cosmwasm_std::Decimal;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use cw_utils::one_coin;

#[cfg(not(feature = "library"))]
pub fn execute_swap(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: SwapMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();
    let config = CONFIG.load(deps.storage)?;

    let coin = one_coin(&info)?;

    if !config.denoms_same_origin.contains(&coin.denom) {
        // TODO: error
    }

    let fee = Decimal::new(coin.amount)
        .checked_mul(config.fee.commission_rate)?
        .to_uint_floor();
    let fee_subtracted = coin.amount.checked_sub(fee)?;

    let lp_allocation = Decimal::new(fee)
        .checked_mul(config.fee.lp_fee_weight)?
        .to_uint_floor();
    let non_lp_allocation = fee.checked_sub(lp_allocation)?;

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
        to_address: config.treasury.to_string(),
        amount: vec![Coin {
            denom: coin.denom,
            amount: non_lp_allocation,
        }],
    }));

    Ok(response)
}
