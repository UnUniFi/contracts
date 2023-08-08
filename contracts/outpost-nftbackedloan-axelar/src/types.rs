use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
}

#[cw_serde]
pub struct DepositInfo {
    pub sender: Addr,
    pub amount: Uint128,
}
