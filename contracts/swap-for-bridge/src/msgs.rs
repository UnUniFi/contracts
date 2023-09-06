use crate::types::Params;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub authority: String,
    pub denoms_same_origin: Vec<String>,
    pub fee_collector: String,
    pub fee_rate: Decimal,
    pub min_fee: Option<Uint128>,
    pub max_fee: Option<Uint128>,
    pub lp_fee_weight: Decimal,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateParams(UpdateParamsMsg),
    Swap(SwapMsg),
    DepositLiquidity(DepositLiquidityMsg),
    WithdrawLiquidity(WithdrawLiquidityMsg),
}

#[cw_serde]
pub struct UpdateParamsMsg {
    pub authority: Option<String>,
    pub denoms_same_origin: Option<Vec<String>>,
    pub fee_collector: Option<String>,
    pub fee_rate: Option<Decimal>,
    pub min_fee: Option<Option<Uint128>>,
    pub max_fee: Option<Option<Uint128>>,
    pub lp_fee_weight: Option<Decimal>,
}

#[cw_serde]
pub struct SwapMsg {
    pub output_denom: String,
    pub recipient: Option<String>,
}

#[cw_serde]
pub struct DepositLiquidityMsg {}

#[cw_serde]
pub struct WithdrawLiquidityMsg {
    pub output_denom: String,
    pub share_amount: Uint128,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Params)]
    Params {},
    #[returns(ShareResp)]
    Share { address: String },
    #[returns(TotalShareResp)]
    TotalShare {},
    #[returns(EstimateFeeResp)]
    EstimateFee { amount: Uint128 },
}

#[cw_serde]
pub struct ShareResp {
    pub share: Uint128,
}

#[cw_serde]
pub struct TotalShareResp {
    pub total_share: Uint128,
}

#[cw_serde]
pub struct EstimateFeeResp {
    pub fee: Uint128,
    pub lp_fee: Uint128,
    pub total_fee: Uint128,
}

#[cw_serde]
pub struct MigrateMsg {}
