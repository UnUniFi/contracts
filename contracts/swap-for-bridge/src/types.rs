use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};

#[cw_serde]
pub struct Params {
    pub authority: Addr,
    pub denoms_same_origin: Vec<String>,
    pub fee_collector: Addr,
    pub fee_rate: Decimal,
    pub min_fee: Option<Uint128>,
    pub max_fee: Option<Uint128>,
    pub lp_fee_weight: Decimal,
}
