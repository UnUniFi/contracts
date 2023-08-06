use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, IbcEndpoint, Uint128};

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

#[cw_serde]
pub struct ChannelInfo {
    /// id of this channel
    pub id: String,
    /// the remote channel/port we connect to
    pub counterparty_endpoint: IbcEndpoint,
    /// the connection this exists on (you can use to query client/consensus info)
    pub connection_id: String,
    /// interchain account address
    pub address: String,
}

/// Metadata defines a set of protocol specific data encoded into the ICS27 channel version bytestring
/// See ICS004: <https://github.com/cosmos/ibc/tree/master/spec/core/ics-004-channel-and-packet-semantics#Versioning>
#[cw_serde]
pub struct Metadata {
    /// version defines the ICS27 protocol version
    pub version: String,
    /// controller_connection_id is the connection identifier associated with the controller chain
    pub controller_connection_id: String,
    /// host_connection_id is the connection identifier associated with the host chain
    pub host_connection_id: String,
    /// address defines the interchain account address to be fulfilled upon the OnChanOpenTry handshake step
    /// NOTE: the address field is empty on the OnChanOpenInit handshake step
    pub address: String,
    /// encoding defines the supported codec format
    pub encoding: String,
    /// tx_type defines the type of transactions the interchain account can execute
    pub tx_type: String,
}

#[cw_serde]
pub struct InterchainAccountPacketData {
    pub r#type: i32,
    pub data: Vec<u8>,
    pub memo: String,
}
