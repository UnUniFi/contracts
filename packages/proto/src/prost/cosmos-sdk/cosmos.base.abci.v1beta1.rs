// @generated
/// ABCIMessageLog defines a structure containing an indexed tx ABCI message log.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbciMessageLog {
    #[prost(uint32, tag = "1")]
    pub msg_index: u32,
    #[prost(string, tag = "2")]
    pub log: ::prost::alloc::string::String,
    /// Events contains a slice of Event objects that were emitted during some
    /// execution.
    #[prost(message, repeated, tag = "3")]
    pub events: ::prost::alloc::vec::Vec<StringEvent>,
}
/// StringEvent defines en Event object wrapper where all the attributes
/// contain key/value pairs that are strings instead of raw bytes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringEvent {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub attributes: ::prost::alloc::vec::Vec<Attribute>,
}
/// Attribute defines an attribute wrapper where the key and value are
/// strings instead of raw bytes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attribute {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// GasInfo defines tx execution gas context.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GasInfo {
    /// GasWanted is the maximum units of work we allow this tx to perform.
    #[prost(uint64, tag = "1")]
    pub gas_wanted: u64,
    /// GasUsed is the amount of gas actually consumed.
    #[prost(uint64, tag = "2")]
    pub gas_used: u64,
}
/// MsgData defines the data returned in a Result object during message
/// execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgData {
    #[prost(string, tag = "1")]
    pub msg_type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}

// https://github.com/cosmos/cosmos-rust/releases/tag/cosmrs%2Fv0.7.1
// https://github.com/cosmos/cosmos-rust/blob/be641f4ec15f7229b012e86f5db2e6a437ea8a5b/cosmos-sdk-proto/src/prost/cosmos-sdk/cosmos.base.abci.v1beta1.rs
/// TxMsgData defines a list of MsgData. A transaction will have a MsgData object
/// for each message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxMsgData {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<MsgData>,
}

// /// TxMsgData defines a list of MsgData. A transaction will have a MsgData object
// /// for each message.
// #[derive(Clone, PartialEq, ::prost::Message)]
// pub struct TxMsgData {
//     /// data field is deprecated and not populated.
//     // #[deprecated]
//     #[prost(message, repeated, tag = "1")]
//     pub data: ::prost::alloc::vec::Vec<MsgData>,
//     /// msg_responses contains the Msg handler responses packed into Anys.
//     ///
//     /// Since: cosmos-sdk 0.46
//     #[prost(message, repeated, tag = "2")]
//     pub msg_responses: ::prost::alloc::vec::Vec<::prost_types::Any>,
// }
// @@protoc_insertion_point(module)
