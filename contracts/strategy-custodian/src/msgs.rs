use crate::types::Params;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use std::time::Duration;
use strategy::v0::msgs::{BondedResp, FeeResp, StakeMsg, UnbondingResp, UnstakeMsg};

#[cw_serde]
pub struct InstantiateMsg {
    pub deposit_denom: String,
    pub unbonding_period: Duration,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateParams(UpdateParamsMsg),
    Stake(StakeMsg),
    Unstake(UnstakeMsg),
    SendBack(SendBackMsg),
    ReportProfit(ReportProfitMsg),
}

#[cw_serde]
pub struct UpdateParamsMsg {
    pub authority: Option<String>,
    pub deposit_denom: Option<String>,
    pub unbonding_period: Option<Duration>,
}

#[cw_serde]
pub struct SendBackMsg {}

#[cw_serde]
pub struct ReportProfitMsg {
    pub profit: Uint128,
    pub sign: bool,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Params)]
    Config {},
    #[returns(BondedResp)]
    Bonded { addr: String },
    #[returns(UnbondingResp)]
    Unbonding { addr: String },
    #[returns(FeeResp)]
    Fee {},
}

/// We currently take no arguments for migrations
#[cw_serde]
pub struct MigrateMsg {}
