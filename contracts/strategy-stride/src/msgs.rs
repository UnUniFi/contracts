use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;
use strategy::v1::msgs::{EpochMsg, StakeMsg, UnstakeMsg};

#[cw_serde]
pub struct InstantiateMsg {
    pub chain_id: String,
    pub deposit_denom: String,
    pub ls_denom: String,
    pub connection_id: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateParams(UpdateParamsMsg),
    Stake(StakeMsg),
    Unstake(UnstakeMsg),
    Epoch(EpochMsg),
}

#[cw_serde]
pub struct UpdateParamsMsg {
    pub authority: Option<String>,
    pub ls_denom: Option<String>,
    pub deposit_denom: Option<String>,
    pub chain_id: Option<String>,
    pub connection_id: Option<String>,
}

#[cw_serde]
pub struct IcqBalanceCallbackMsg {
    pub coins: Vec<Coin>,
}

#[cw_serde]
pub struct IbcTransferCallbackMsg {}

#[cw_serde]
pub struct IbcTransferToHostMsg {}

#[cw_serde]
pub struct IbcTransferToControllerMsg {}

#[cw_serde]
pub enum QueryMsg {
    Version {},
    DepositDenom {},
    Fee {},
    Amounts { addr: String },
    Kyc {},
    Params {},
    State {},
    Bonded { addr: String },
    Unbonding { addr: String },
    Unbondings {},
}

/// We currently take no arguments for migrations
#[cw_serde]
pub struct MigrateMsg {}
