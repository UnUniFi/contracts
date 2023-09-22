use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Timestamp, Uint128};

#[cw_serde]
pub struct Params {
    pub authority: Addr,
}

#[cw_serde]
pub struct BonusWindow {
    pub id: u64,
    pub denom: String,
    pub budget_for_all: Uint128,
    pub apr_for_winners: Vec<Decimal>,
    pub start_at: Timestamp,
    pub end_at: Timestamp,
}

#[cw_serde]
pub struct VotedVault {
    pub bonus_window_id: u64,
    pub vault_id: u64,
    pub voted_amount: Uint128,
}

#[cw_serde]
pub struct VaultShareStaking {
    pub vault_id: u64,
    pub address: Addr,
    pub vault_share: Uint128,
    pub start_at: Timestamp,
}
