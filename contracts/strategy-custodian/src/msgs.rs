use crate::types::Params;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{CosmosMsg, Decimal, Uint128};
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
    Admin(AdminMsg),
    SendBack(SendBackMsg),
    ReportProfit(ReportProfitMsg),
}

#[cw_serde]
pub struct UpdateParamsMsg {
    pub admin: Option<String>,
    pub deposit_denom: Option<String>,
    pub performance_fee_rate: Option<Decimal>,
    pub withdraw_fee_rate: Option<Decimal>,
    pub min_withdraw_fee: Option<Option<Uint128>>,
    pub max_withdraw_fee: Option<Option<Uint128>>,
    pub trusted_kyc_provider_ids: Option<Vec<u64>>,
}

#[cw_serde]
pub struct AdminMsg {
    pub cosmos_msgs: Vec<CosmosMsg>,
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
    #[returns(SendBackAmountResp)]
    SendBackAmount {},
}

#[cw_serde]
pub struct SendBackAmountResp {
    pub amount: Uint128,
    pub share: Uint128,
}

/// We currently take no arguments for migrations
#[cw_serde]
pub struct MigrateMsg {}
