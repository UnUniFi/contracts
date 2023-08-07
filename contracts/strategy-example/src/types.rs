use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub unbond_period: u64,
    pub deposit_denom: String,
    pub redemption_rate: Uint128,
    pub total_deposit: Uint128,
}

#[cw_serde]
pub struct DepositInfo {
    pub sender: Addr,
    pub amount: Uint128,
}
