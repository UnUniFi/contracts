use crate::state::{Config, CONFIG, HOST_LP_RATE_MULTIPLIER};
use cosmwasm_std::{Deps, StdResult, Uint128};

use super::unbondings::{query_unbondings, UNBONDING_ITEM_LIMIT};

pub fn query_unbonding(deps: Deps, addr: String) -> StdResult<Uint128> {
    let config: Config = CONFIG.load(deps.storage)?;
    let mut pending_unbonding_lp = Uint128::new(0u128);
    let unbondings = query_unbondings(deps.storage, Some(UNBONDING_ITEM_LIMIT))?;
    for unbonding in unbondings {
        if unbonding.sender == addr {
            pending_unbonding_lp += unbonding.amount;
        }
    }
    Ok(pending_unbonding_lp * config.host_config.lp_redemption_rate / HOST_LP_RATE_MULTIPLIER)
}
