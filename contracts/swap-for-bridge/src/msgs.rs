use crate::types::{Config, FeeConfig};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    pub authority: String,
    pub treasury: String,
    pub denoms_same_origin: Vec<String>,
    pub fee: FeeConfig,
}

#[cw_serde]
pub enum ExecuteMsg {
    Swap(SwapMsg),
    DepositLiquidity(DepositLiquidityMsg),
    WithdrawLiquidity(WithdrawLiquidityMsg),
    UpdateConfig(UpdateConfigMsg),
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
pub struct UpdateConfigMsg {
    pub authority: Option<String>,
    pub treasury: Option<String>,
    pub denoms_same_origin: Option<Vec<String>>,
    pub fee: Option<FeeConfig>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},
}

#[cw_serde]
pub struct MigrateMsg {}
