use crate::msgs::{ChannelInfo, Phase, PhaseStep};
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

    pub chain_id: String,
    pub pool_id: u64,     // 1 for ATOM/OSMO
    pub lp_denom: String, // "gamm/pool/1" for ATOM/OSMO
    pub bonded_lp_amount: Uint128,
    pub unbonding_lp_amount: Uint128,
    pub free_lp_amount: Uint128,
    pub pending_bond_lp_amount: Uint128,
    pub pending_lp_removal_amount: Uint128, // pending removal from lp
    pub lp_redemption_rate: Uint128,
    pub lock_id: u64,

    pub quote_denom: String, // OSMO for ATOM/OSMO
    pub free_quote_amount: Uint128,

    pub base_denom: String,        // ATOM for ATOM/OSMO
    pub free_base_amount: Uint128, // free ATOM balance
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub unbond_period: u64,
    pub redemption_rate: Uint128,
    pub total_shares: Uint128,
    pub total_deposit: Uint128,
    pub total_withdrawn: Uint128,
    pub last_unbonding_id: u64,
    pub phase: Phase,
    pub phase_step: PhaseStep, // counted from 1 for each phase
    pub pending_icq: u64,

    pub ica_channel_id: String,
    pub ica_connection_id: String,
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
    pub amount: Uint128,     // lp amount at the ratio
    pub pending_start: bool, // ica message broadcasted for withdrawal and waiting for callback
    pub start_time: u64,
    pub marked: bool, // flag for withdrawal phase withdraw
}

pub const UNBONDINGS: Map<u64, Unbonding> = Map::new("unbondings");
pub const DEPOSITS: Map<String, DepositInfo> = Map::new("deposits");

pub const CHANNEL_INFO: Map<&str, ChannelInfo> = Map::new("channel_info");

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

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct IcaAmounts {
    pub to_swap_base: Uint128,
    pub to_swap_quote: Uint128,
    pub to_remove_lp: Uint128,
    pub to_transfer_to_controller: Uint128,
    pub to_transfer_to_host: Uint128,
    pub to_return_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum EpochCallSource {
    NormalEpoch,
    IcqCallback,
    IcaCallback,
    TransferCallback,
}

pub const STAKE_RATE_MULTIPLIER: Uint128 = Uint128::new(1000000u128); // 10^6
pub const HOST_LP_RATE_MULTIPLIER: Uint128 = Uint128::new(1000000_000000_000000u128); // 10^18
