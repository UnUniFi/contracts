use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use std::collections::BTreeMap;

#[cw_serde]
pub struct Params {
    pub authority: Addr,
    pub symbol_swap_contract_map: BTreeMap<String, Addr>,
}
