use crate::state::VOTED_VAULTS;
use crate::types::VotedVault;
use cosmwasm_std::Order;
use cosmwasm_std::{Deps, StdResult};

#[cfg(not(feature = "library"))]
pub fn query_voted_vaults(deps: Deps, bonus_window_id: u64) -> StdResult<Vec<VotedVault>> {
    let voted_vaults = VOTED_VAULTS
        .prefix(bonus_window_id)
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item_result| -> StdResult<_> { 
            let item = item_result?;
            let vault_id = item.0.to_owned();
            let voted_amount = item.1.clone();
            Ok( VotedVault { vault_id: vault_id, voted_amount: voted_amount} )
        })
        .collect::<StdResult<Vec<_>>>()?;

    Ok(voted_vaults)
}
