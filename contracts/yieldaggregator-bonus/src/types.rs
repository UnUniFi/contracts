use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128, Decimal};

#[cw_serde]
pub struct Params {
    pub authority: Addr,
}

#[cw_serde]
pub struct BonusWindow {
    pub id: u64,
    pub denom: String,
    pub budget_for_all: Uint128,
    pub reward_for_winners: Vec<Uint128>,
    pub start_at: Timestamp,
    pub end_at: Timestamp,
}

#[cw_serde]
pub struct VaultShareStaking {
    pub vault_share: Uint128,
    pub staking_power_index: Decimal,
}

#[cw_serde]
pub struct VotedVault {
    pub vault_id: u64,
    pub voted_amount: Uint128,
}

#[cw_serde]
pub struct TotalStakingPowerIndexForVault {
    pub vault_id: u64,
    pub total_staking_power_index: Uint128,
}
