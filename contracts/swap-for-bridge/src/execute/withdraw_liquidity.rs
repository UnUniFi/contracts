use crate::error::ContractError;
use crate::msgs::WithdrawLiquidityMsg;
use crate::state::PARAMS;
use crate::state::TOTAL_SHARE;
use crate::{balance::get_total_amounts, state::SHARES};
use cosmwasm_std::Coin;
use cosmwasm_std::Decimal;
use cosmwasm_std::Uint128;
use cosmwasm_std::{BankMsg, CosmosMsg};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw_utils::nonpayable;

#[cfg(not(feature = "library"))]
pub fn execute_withdraw_liquidity(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: WithdrawLiquidityMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();

    nonpayable(&info)?;

    let config = PARAMS.load(deps.storage)?;

    if !config.denoms_same_origin.contains(&msg.output_denom) {
        return Err(ContractError::InvalidDenom);
    }

    let owned_share = SHARES
        .may_load(deps.storage, info.sender.clone())?
        .unwrap_or_else(|| Uint128::new(0));
    if owned_share < msg.share_amount {
        return Err(ContractError::InsufficientFunds);
    }

    let total_share = TOTAL_SHARE.load(deps.storage)?;
    let total_token_amount = get_total_amounts(
        deps.as_ref(),
        env.contract.address,
        config.denoms_same_origin,
    )?;

    // total_share : total_token_amount = share_amount : token_amount
    let token_amount = if total_share.is_zero() {
        Uint128::new(0)
    } else {
        total_token_amount
            .checked_mul(msg.share_amount)?
            .checked_div(total_share)
            .unwrap()
    };

    let new_total_share = total_share.checked_sub(msg.share_amount)?;
    TOTAL_SHARE.save(deps.storage, &new_total_share)?;

    let new_share = owned_share.checked_sub(msg.share_amount)?;
    SHARES.save(deps.storage, info.sender.clone(), &new_share)?;

    let fee = Decimal::from_atomics(token_amount, 0)?
        .checked_mul(config.fee_rate)?
        .to_uint_floor();
    let fee_subtracted = token_amount.checked_sub(fee)?;

    response = response.add_message(CosmosMsg::Bank(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![Coin {
            denom: msg.output_denom.clone(),
            amount: fee_subtracted,
        }],
    }));

    response = response.add_message(CosmosMsg::Bank(BankMsg::Send {
        to_address: config.fee_collector.to_string(),
        amount: vec![Coin {
            denom: msg.output_denom,
            amount: fee,
        }],
    }));

    response = response.add_attribute("action", "withdraw_liquidity");

    Ok(response)
}
