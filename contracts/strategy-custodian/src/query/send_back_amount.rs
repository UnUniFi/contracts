use crate::msgs::SendBackAmountResp;
use cosmwasm_std::{Deps, StdResult, Uint128};

#[cfg(not(feature = "library"))]
pub fn query_send_back_amount(deps: Deps) -> StdResult<SendBackAmountResp> {
    use crate::state::{TOTAL_DEPOSIT, TOTAL_SHARE, TOTAL_UNBONDING_SHARE};

    let total_deposit = TOTAL_DEPOSIT.load(deps.storage)?;
    let total_share = TOTAL_SHARE.load(deps.storage)?;

    let total_unbonding_share = TOTAL_UNBONDING_SHARE.load(deps.storage)?;

    let total_unbonding_deposit = if total_share.is_zero() {
        Uint128::zero()
    } else {
        total_unbonding_share
            .checked_mul(total_deposit)?
            .checked_div(total_share)
            .unwrap()
    };

    Ok(SendBackAmountResp {
        amount: total_unbonding_deposit,
        share: total_unbonding_share,
    })
}
