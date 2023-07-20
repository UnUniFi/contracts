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

    pub pool_id: u64,     // 1
    pub lp_denom: String, // ATOM-OSMO
    pub bonded_lp_amount: Uint128,
    pub free_lp_amount: Uint128,
    pub pending_bond_lp_amount: Uint128,
    pub pending_lp_removal_amount: Uint128, // pending removal from lp
    pub lp_redemption_rate: Uint128,
    pub lock_id: u64,

    pub osmo_denom: String, // OSMO
    pub free_osmo_amount: Uint128,
    pub pending_swap_to_atom_amount: Uint128, // pending swap from OSMO to ATOM

    pub atom_denom: String,                    // ATOM
    pub free_atom_amount: Uint128,             // free ATOM balance
    pub pending_swap_to_osmo_amount: Uint128,  // pending swap from ATOM -> OSMO to add liquidity
    pub pending_add_liquidity_amount: Uint128, // amount of ATOM used on liquidity addition
    pub pending_transfer_amount: Uint128, // pending transfer to controller - TODO: how to get hook for transfer finalization?
                                          // TODO: probably create two ica accounts for convenience
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Phase {
    Deposit,
    Withdraw,
}

// Regular epoch operation (once per day)
// - icq balance of ica account when `Deposit` phase
// Unbonding epoch operation
// - begin lp unbonding on host through ica tx per unbonding epoch - per day probably - (if to unbond lp is not enough, wait for icq to update bonded lp correctly)
// `Deposit` phase operations
// - This phase starts when `WithdrawToUser` phase ends
// - ibc transfer to host for newly incoming atoms
// - ibc transfer to host for stacked atoms during withdraw phases
// - swap half atom to osmo & half osmo to atom in a single ica tx
// - initiate and wait for icq to update latest balances
// - add liquidity & bond in a single ica tx
// - repeat the flow
// `DepositEnding` phase operations
// - This phase starts from `Deposit` phase, when ica free lp balance is positive
// - ibc transfers are disabled
// - swap half atom to osmo & half osmo to atom in a single ica tx
// - wait for icq to update latest balances
// - add liquidity & bond in a single ica tx
// - initiate and wait for icq to update latest balances
// - update to phase to `LqWithdraw`
// `Withdraw` phase operations
// - This phase starts when `DepositEnding` phase ends
// - Mark unbond ending queue items on contract
// - execute remove liquidity operation
// - initiate and wait or icq to update latest balances
// - swap full osmo to atom
// - initiate and wait or icq to update latest balances
// - ibc transfer full atom balance from ica to contract
// - wait for ica callback for ibc transfer finalization
// - calculate amount to return, contract balance - stacked atom balance for deposit
// - send amounts to marked unbond ending items proportionally
// - switch to `Deposit` phase
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub unbond_period: u64,
    pub redemption_rate: Uint128,
    pub total_deposit: Uint128,
    pub total_withdrawn: Uint128,
    pub last_unbonding_id: u64,
    pub phase: Phase,
    pub phase_step: u64, // counted from 1 for each phase
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
