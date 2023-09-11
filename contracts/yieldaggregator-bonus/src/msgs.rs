use crate::types::{BonusWindow, Params, VaultShareStaking, VotedVault};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Decimal, Timestamp, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub authority: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateParams(UpdateParamsMsg),
    RegisterBonusWindow(RegisterBonusWindowMsg),
    DeleteBonusWindow(DeleteBonusWindowMsg),
    Vote(VoteMsg),
    StakeVaultShare(StakeVaultShareMsg),
}

#[cw_serde]
pub struct UpdateParamsMsg {
    pub authority: Option<String>,
}

#[cw_serde]
pub struct RegisterBonusWindowMsg {
    pub denom: String,
    pub budget_for_all: Uint128,
    pub apr_for_winners: Vec<Decimal>,
    pub start_at: Timestamp,
    pub end_at: Timestamp,
}

#[cw_serde]
pub struct DeleteBonusWindowMsg {
    pub bonus_window_id: u64,
}

#[cw_serde]
pub struct AmountForWinner {
    pub vault_id: u64,
    pub amount: Uint128,
}

#[cw_serde]
pub struct VoteMsg {
    pub bonus_window_id: u64,
    pub vault_id: u64,
}

#[cw_serde]
pub struct StakeVaultShareMsg {
    pub bonus_window_id: u64,
    pub vault_id: u64,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Params)]
    Params {},
    #[returns(Vec<BonusWindow>)]
    BonusWindows {},
    #[returns(Vec<VotedVault>)]
    VotedVaults { bonus_window_id: u64 },
    #[returns(Vec<VaultShareStaking>)]
    VaultShareStaking {
        bonus_window_id: u64,
        address: String,
    },
    #[returns(DistributionAmountResp)]
    DistributionAmount { bonus_window_id: u64 },
}

#[cw_serde]
pub struct DistributionAmountResp {
    bonus_window_id: u64,
    for_all: Vec<Distribution>,
    for_winners: Vec<Distribution>,
}

#[cw_serde]
pub struct Distribution {
    recipient: Addr,
    amount: Coin,
}

/// We currently take no arguments for migrations
#[cw_serde]
pub struct MigrateMsg {}
