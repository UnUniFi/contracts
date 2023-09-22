use crate::state::VAULT_SHARE_STAKINGS;
use crate::types::VaultShareStaking;
use cosmwasm_std::{Deps, StdResult};

#[cfg(not(feature = "library"))]
pub fn query_vault_share_staking(
    deps: Deps,
    bonus_window_id: u64,
    address: String,
) -> StdResult<VaultShareStaking> {
    let address = deps.api.addr_validate(&address)?;
    let vault_share_staking =
        VAULT_SHARE_STAKINGS.load(deps.storage, (bonus_window_id, address))?;

    Ok(vault_share_staking)
}
