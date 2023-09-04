use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};

#[cw_serde]
pub struct Params {
    pub authority: Addr,
    pub denoms_same_origin: Vec<String>,
    pub fee_collector: Addr,
    pub fee_rate: Decimal,
    pub lp_fee_rate: Decimal,
}
