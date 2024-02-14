use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Params {
    pub authority: Addr,
    pub chain_id: String,
    pub denom: String,           // `uatom`
    pub ls_denom: String,        // `ibc/xxxxstuatom`
    pub ls_denom_native: String, // `ibc/xxxxstuatom`
    pub ls_rate_feeder: String,  // redemption rate feeder
    pub connection_id: String,
}
pub const PARAMS: Item<Params> = Item::new("params");

#[cw_serde]
pub struct State {
    pub total_amount: Uint128,
    pub total_deposit: Uint128,
    pub total_withdrawn: Uint128,
    pub ls_redemption_rate: Decimal,
}

pub const STATE: Item<State> = Item::new("state");

#[cw_serde]
pub struct DepositInfo {
    pub sender: Addr,
    pub amount: Uint128, // contract deposit ratio
}

pub const DEPOSITS: Map<String, DepositInfo> = Map::new("deposits");
pub const LS_RATE_MULTIPLIER: Uint128 = Uint128::new(1000000_000000_000000u128); // 10^18
