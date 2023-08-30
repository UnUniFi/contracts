use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct Params {
    pub authority: Addr,
    pub deposit_denom: String,
}

#[cw_serde]
pub struct Bonded {
    pub address: Addr,
    pub share: Uint128,
}

#[cw_serde]
pub struct Unbonding {
    pub address: Addr,
    pub share: Uint128,
}
