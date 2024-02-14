use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal};
use strategy::v1::msgs::{EpochMsg, StakeMsg, UnstakeMsg};

#[cw_serde]
pub struct InstantiateMsg {
    pub chain_id: String,
    pub denom: String,
    pub ls_denom: String,
    pub ls_denom_native: String,
    pub ls_rate_feeder: String,
    pub connection_id: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateParams(UpdateParamsMsg),
    Stake(StakeMsg),
    Unstake(UnstakeMsg),
    Epoch(EpochMsg),
    SetRedemptionRate(SetRedemptionRateMsg),
}

#[cw_serde]
pub struct UpdateParamsMsg {
    pub authority: Option<String>,
    pub ls_rate_feeder: Option<String>,
    pub ls_denom: Option<String>,
    pub ls_denom_native: Option<String>,
    pub denom: Option<String>,
    pub chain_id: Option<String>,
    pub connection_id: Option<String>,
}

#[cw_serde]
pub struct IcqBalanceCallbackMsg {
    pub coins: Vec<Coin>,
}

#[cw_serde]
pub struct IbcTransferCallbackMsg {}

#[cw_serde]
pub struct IbcTransferToHostMsg {}

#[cw_serde]
pub struct IbcTransferToControllerMsg {}

#[cw_serde]
pub struct SetRedemptionRateMsg {
    pub ls_rate: Decimal,
}

#[cw_serde]
pub enum QueryMsg {
    Version {},
    DepositDenom {},
    Fee {},
    Amounts { addr: String },
    Kyc {},
    Params {},
    State {},
    Bonded { addr: String },
    Unbonding { addr: String },
    Unbondings {},
}

/// We currently take no arguments for migrations
#[cw_serde]
pub struct MigrateMsg {}
