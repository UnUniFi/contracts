use crate::types::Params;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Uint128};
use strategy::v1::msgs::{
    AmountsResp, DepositDenomResp, FeeResp, KycResp, StakeMsg, UnstakeMsg, VersionResp,
};

#[cw_serde]
pub struct InstantiateMsg {
    pub deposit_denom: String,
    pub performance_fee_rate: Decimal,
    pub withdraw_fee_rate: Decimal,
    pub min_withdraw_fee: Option<Uint128>,
    pub max_withdraw_fee: Option<Uint128>,
    pub trusted_kyc_provider_ids: Vec<u64>,
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
    Params {},
    #[returns(VersionResp)]
    Version {},
    #[returns(DepositDenomResp)]
    DepositDenom {},
    #[returns(AmountsResp)]
    Amounts { addr: String },
    #[returns(FeeResp)]
    Fee {},
    #[returns(KycResp)]
    Kyc {},
}

/// We currently take no arguments for migrations
#[cw_serde]
pub struct MigrateMsg {}
