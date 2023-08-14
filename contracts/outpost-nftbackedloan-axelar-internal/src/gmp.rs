use cosmwasm_schema::cw_serde;
use proto::{cosmos::base::v1beta1::Coin, traits::TypeUrl};

#[cw_serde]
pub struct Fee {
    pub amount: String,
    pub recipient: String,
}

#[cw_serde]
pub struct GmpMessage {
    pub destination_chain: String,
    pub destination_address: String,
    pub payload: Vec<u8>,
    #[serde(rename = "type")]
    pub type_: i64,
    pub fee: Option<Fee>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IbcCounterpartyHeight {
    #[prost(uint64, optional, tag = "1")]
    revision_number: Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    revision_height: Option<u64>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransfer {
    #[prost(string, tag = "1")]
    pub source_port: String,
    #[prost(string, tag = "2")]
    pub source_channel: String,
    #[prost(message, optional, tag = "3")]
    pub token: ::core::option::Option<Coin>,
    #[prost(string, tag = "4")]
    pub sender: String,
    #[prost(string, tag = "5")]
    pub receiver: String,
    #[prost(message, optional, tag = "6")]
    pub timeout_height: Option<IbcCounterpartyHeight>,
    #[prost(uint64, optional, tag = "7")]
    pub timeout_timestamp: ::core::option::Option<u64>,
    #[prost(string, tag = "8")]
    pub memo: String,
}

impl TypeUrl for MsgTransfer {
    const TYPE_URL: &'static str = "/ibc.applications.transfer.v1.MsgTransfer";
}

// We define the response as a prost message to be able to decode the protobuf data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferResponse {
    #[prost(uint64, tag = "1")]
    pub sequence: u64,
}
