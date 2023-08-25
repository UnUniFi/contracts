use crate::error::ContractError;
use crate::msgs::DepositLiquidityMsg;
use crate::state::CONFIG;
use crate::state::TOTAL_SHARE;
use cosmwasm_std::Response;
use cosmwasm_std::Uint128;
use cosmwasm_std::{DepsMut, Env, MessageInfo};
use cw_utils::one_coin;

#[cfg(not(feature = "library"))]
pub fn execute_deposit_liquidity(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: DepositLiquidityMsg,
) -> Result<Response, ContractError> {
    use crate::state::SHARES;

    let mut response = Response::new();
    let config = CONFIG.load(deps.storage)?;

    let coin = one_coin(&info)?;

    if !config.denoms_same_origin.contains(&coin.denom) {
        return Err(ContractError::InvalidDenom);
    }

    let total_share = TOTAL_SHARE.load(deps.storage)?;
    let total_token_amount = Uint128::new(0);

    // total_share : total_token_amount = share_amount : token_amount
    let share_amount = if total_token_amount.is_zero() {
        coin.amount
    } else {
        total_share
            .checked_mul(coin.amount)?
            .checked_div(total_token_amount)
            .unwrap()
    };

    let new_total_share = total_share.checked_add(share_amount)?;
    TOTAL_SHARE.save(deps.storage, &new_total_share)?;

    let new_share = match SHARES.may_load(deps.storage, info.sender.clone())? {
        Some(share) => share.checked_add(share_amount)?,
        None => share_amount,
    };
    SHARES.save(deps.storage, info.sender, &new_share)?;

    response = response.add_attribute("action", "deposit_liquidity");

    Ok(response)
}
