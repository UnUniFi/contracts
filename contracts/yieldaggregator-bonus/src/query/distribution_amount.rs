use crate::msgs::DistributionAmountResp;
use crate::state::BONUS_WINDOWS;
use cosmwasm_std::{Deps, StdResult};

#[cfg(not(feature = "library"))]
pub fn query_distribution_amount(
    deps: Deps,
    bonus_window_id: u64,
) -> StdResult<DistributionAmountResp> {
    let bonus_window = BONUS_WINDOWS.load(deps.storage, bonus_window_id)?;

    let resp = DistributionAmountResp {};
    Ok(resp)
}
