use crate::state::{VAULT_SHARE_STAKINGS, TOTAL_STAKING_INFO};
use crate::types::VaultShareStaking;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Deps, StdResult, Order, Decimal, Uint128};

#[cfg(not(feature = "library"))]
pub fn query_vault_share_staking(
    deps: Deps,
    bonus_window_id: u64,
    vault_id: u64,
    address: String,
) -> StdResult<VaultShareStaking> {
    let address = deps.api.addr_validate(&address)?;
    let vault_share_staking =
        VAULT_SHARE_STAKINGS.load(deps.storage, (bonus_window_id, vault_id, address))?;

    Ok(vault_share_staking)
}

#[cfg(not(feature = "library"))]
pub fn query_total_staking_info_for_a_vault(
    deps: Deps,
    bonus_window_id: u64,
    vault_id: u64,
) -> StdResult<TotalStakingInfo> {
    let total_staking_info = TOTAL_STAKING_INFO.load(deps.storage, (bonus_window_id, vault_id))?;
    
    Ok( TotalStakingInfo { 
        vault_id: vault_id, 
        total_staked_amount: total_staking_info.0, 
        total_staking_power_index: total_staking_info.1
     })
}

#[cfg(not(feature = "library"))]
pub fn query_total_staking_info(
    deps: Deps,
    bonus_window_id: u64,
) -> StdResult<Vec<TotalStakingInfo>> {
    let total_staking_info = TOTAL_STAKING_INFO
        .prefix(bonus_window_id)
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item_result| -> StdResult<_> {
            let (vault_id, vault_staking_info) = item_result?;
            Ok( TotalStakingInfo {
                vault_id: vault_id,
                total_staked_amount: vault_staking_info.0,
                total_staking_power_index: vault_staking_info.1,
            })
        })
        .collect::<StdResult<Vec<TotalStakingInfo>>>()?;

    Ok(total_staking_info)
}

#[cw_serde]
pub struct  TotalStakingInfo {
    pub vault_id: u64,
    pub total_staked_amount: Uint128,
    pub total_staking_power_index: Decimal,
}