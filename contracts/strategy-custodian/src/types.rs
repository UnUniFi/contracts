use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};

#[cw_serde]
pub struct Params {
    pub authority: Addr,
    pub deposit_denom: String,
    pub performance_fee_rate: Decimal,
    pub withdraw_fee_rate: Decimal,
    pub min_withdraw_fee: Option<Uint128>,
    pub max_withdraw_fee: Option<Uint128>,
    pub trusted_kyc_provider_ids: Vec<u64>,
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
