use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Decimal, Uint128};

#[cw_serde]
pub enum ExecuteMsg {
    Stake(StakeMsg),
    Unstake(UnstakeMsg),
    ExecuteEpoch(ExecuteEpochMsg),
}

#[cw_serde]
pub struct StakeMsg {}

#[cw_serde]
pub struct UnstakeMsg {
    pub amount: Uint128,
}

#[cw_serde]
pub struct ExecuteEpochMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(BondedResp)]
    Bonded { addr: String },
    #[returns(UnbondingResp)]
    Unbonding { addr: String },
    #[returns(FeeResp)]
    Fee {},
}

pub type BondedResp = Uint128;

pub type UnbondingResp = Uint128;

#[cw_serde]
pub struct FeeResp {
    pub deposit_fee_rate: Decimal,
    pub withdraw_fee_rate: Decimal,
    pub interest_fee_rate: Decimal,
}

#[cw_serde]
pub enum SudoMsg {
    #[serde(rename = "kv_query_result")]
    KVQueryResult {
        connection_id: String,
        chain_id: String,
        query_prefix: String,
        query_key: Binary,
        data: Binary,
    },
    #[serde(rename = "transfer_callback")]
    TransferCallback {
        denom: String,
        amount: String,
        sender: String,
        receiver: String,
        memo: String,
        success: bool,
    },
}
