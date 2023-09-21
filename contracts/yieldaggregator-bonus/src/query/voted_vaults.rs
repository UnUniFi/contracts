use crate::state::VOTED_VAULTS;
use crate::types::VotedVault;
use cosmwasm_std::Order;
use cosmwasm_std::{Deps, StdResult};

#[cfg(not(feature = "library"))]
pub fn query_voted_vaults(deps: Deps, bonus_window_id: u64) -> StdResult<Vec<VotedVault>> {
    let voted_vaults = VOTED_VAULTS
        .prefix(bonus_window_id)
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| -> StdResult<_> { Ok(item?.1) })
        .collect::<StdResult<Vec<_>>>()?;

    Ok(voted_vaults)
}
