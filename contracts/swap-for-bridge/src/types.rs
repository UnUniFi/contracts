use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};

#[cw_serde]
pub struct Config {
    pub denoms_same_origin: Vec<String>,
    pub authority: Addr,
    pub treasury: Addr,
    pub fee: FeeConfig,
}

#[cw_serde]
pub struct FeeConfig {
    pub commission_rate: Decimal,
    pub lp_fee_weight: Decimal,
}
