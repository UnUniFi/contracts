use std::collections::BTreeMap;

use crate::types::Params;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {
    pub authority: String,
    pub denom_swap_contract_map: BTreeMap<String, String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateParams(UpdateParamsMsg),
    DepositToVault(DepositToVaultMsg),
}

#[cw_serde]
pub struct UpdateParamsMsg {
    pub authority: Option<String>,
    pub denom_swap_contract_map: Option<BTreeMap<String, String>>,
}

#[cw_serde]
pub struct DepositToVaultMsg {
    pub depositor: String,
    pub vault_id: u64,
    pub swap_output_denom: Option<String>,
}

#[cw_serde]
pub struct TransferMsg {
    pub source_port: String,
    pub source_channel: String,
    pub token :Coin,
    pub sender: String,
    pub receiver: String,
    pub timeout_timestamp: String,
    pub memo: DepositToVaultMsg
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Params)]
    Params {},
}

/// We currently take no arguments for migrations
#[cw_serde]
pub struct MigrateMsg {}
