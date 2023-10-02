use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Coin, CosmosMsg, CustomMsg, IbcTimeout};

#[cw_serde]
pub enum UnunifiMsg {
    #[serde(rename = "ibc_transfer")]
    IbcTransfer {
        channel_id: String,
        to_address: String,
        amount: Coin,
        timeout: IbcTimeout,
    },
    #[serde(rename = "request_kv_icq")]
    RequestKvIcq {
        connection_id: String,
        chain_id: String,
        query_prefix: String,
        query_key: Binary,
    },
    #[serde(rename = "deputy_deposit_to_vault")]
    DeputyDepositToVault {
        depositor: String,
        vault_id: String,
        amount: Coin,
    },
}

impl From<UnunifiMsg> for CosmosMsg<UnunifiMsg> {
    fn from(msg: UnunifiMsg) -> CosmosMsg<UnunifiMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for UnunifiMsg {}
