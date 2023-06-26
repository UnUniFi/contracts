use cosmwasm_std::{
    Addr, Binary, BlockInfo, IbcEndpoint, Order, StdResult, Storage, Timestamp, Uint128,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ControllerConfig {
    pub transfer_channel_id: String,
    pub deposit_denom: String, // `ibc/xxxxuatom`
    pub free_amount: Uint128,
    pub stacked_amount_to_deposit: Uint128,
    pub pending_transfer_amount: Uint128, // TODO: where to get hook for transfer finalization?
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct HostConfig {
    pub transfer_channel_id: String,

    pub lp_denom: String, // ATOM-OSMO
    pub bonded_lp_amount: Uint128,
    pub unbonding_lp_amount: Uint128,
    pub free_lp_amount: Uint128,
    pub pending_bond_lp_amount: Uint128,
    pub pending_unbond_lp_amount: Uint128,
    pub pending_swap_lp_amount: Uint128, // pending swap from lp to deposit token amount
    pub lp_redemption_rate: Uint128,

    pub osmo_denom: String, // OSMO
    pub free_osmo_amount: Uint128,
    pub pending_swap_to_atom_amount: Uint128, // Convert OSMO to ATOM

    pub atom_denom: String,                    // ATOM
    pub free_atom_amount: Uint128,             // free ATOM balance
    pub pending_swap_to_osmo_amount: Uint128,  // pending swap from ATOM -> OSMO to add liquidity
    pub pending_add_liquidity_amount: Uint128, // amount of ATOM used on liquidity addition
    pub pending_transfer_amount: Uint128, // pending transfer to controller - TODO: how to get hook for transfer finalization?
    pub required_withdrawal_amount: Uint128,
    // TODO: probably create two ica accounts for convenience
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Phase {
    Deposit,
    DepositEnding,
    Withdraw,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub unbond_period: u64,
    pub redemption_rate: Uint128,
    pub total_deposit: Uint128,
    pub total_withdrawn: Uint128,
    pub last_unbonding_id: u64,
    pub phase: Phase,

    pub ica_channel_id: String,
    pub ica_account: String,
    pub transfer_timeout: u64,
    pub host_config: HostConfig,
    pub controller_config: ControllerConfig,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DepositInfo {
    pub sender: Addr,
    pub amount: Uint128, // contract deposit ratio
}

// Unbonding record is removed when bank send is finalized
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Unbonding {
    pub id: u64,
    pub sender: Addr,
    pub amount: Uint128, // lp amount at the ratio
    pub start_time: u64,
    pub marked: bool, // flag for withdrawal phase withdraw
}

pub const UNBONDINGS: Map<u64, Unbonding> = Map::new("unbondings");
pub const DEPOSITS: Map<String, DepositInfo> = Map::new("deposits");

pub const CHANNEL_INFO: Map<&str, ChannelInfo> = Map::new("channel_info");

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
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
#[derive(Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct InterchainAccountPacketData {
    pub r#type: i32,
    pub data: Vec<u8>,
    pub memo: String,
}

pub struct IcaAmounts {
    pub to_swap_atom: Uint128,
    pub to_swap_osmo: Uint128,
    pub to_add_lp: Uint128,
    pub to_remove_lp: Uint128,
    pub to_unbond_lp: Uint128,
    pub to_transfer_to_controller: Uint128,
    pub to_transfer_to_host: Uint128,
    pub to_return_amount: Uint128,
}
