use crate::state::{BONDEDS, TOTAL_DEPOSIT, TOTAL_SHARE, UNBONDINGS};
use cosmwasm_std::{Deps, StdResult, Uint128};
use strategy::v1::msgs::AmountsResp;

#[cfg(not(feature = "library"))]
pub fn query_amounts(deps: Deps, addr: String) -> StdResult<AmountsResp> {
    use crate::types::{Bonded, Unbonding};

    let addr = deps.api.addr_validate(&addr)?;
    let bonded = BONDEDS.may_load(deps.storage, addr.clone())?
        .unwrap_or_else(|| Bonded {
            address: addr.clone(),
            share: Uint128::zero(),
        });
    let unbonding = UNBONDINGS.may_load(deps.storage, addr.clone())?
        .unwrap_or_else(|| Unbonding {
            address: addr.clone(),
            share: Uint128::zero(),
        });

    let total_deposit = TOTAL_DEPOSIT.load(deps.storage)?;
    let total_share = TOTAL_SHARE.load(deps.storage)?;

    let bonded_amount = if total_share.is_zero() {
        Uint128::zero()
    } else {
        bonded.share * total_deposit / total_share
    };

    let unbonding_amount = if total_share.is_zero() {
        Uint128::zero()
    } else {
        unbonding.share * total_deposit / total_share
    };

    Ok(AmountsResp {
        total_deposited: bonded_amount + unbonding_amount,
        bonding_standby: Uint128::new(0),
        bonded: bonded_amount,
        unbonding: unbonding_amount,
    })
}
