use cosmwasm_std::{Deps, StdResult, Uint128};
use strategy::v1::msgs::AmountsResp;

use super::{bonded::query_bonded, unbonding::query_unbonding};

pub fn query_amounts(deps: Deps, addr: String) -> StdResult<AmountsResp> {
    let unbonding = query_unbonding(deps, addr.to_owned())?;
    let bonded = query_bonded(deps, addr.to_owned())?;
    Ok(AmountsResp {
        total_deposited: bonded + unbonding,
        bonding_standby: Uint128::from(0u128),
        bonded: bonded,
        unbonding: unbonding,
    })
}
