// use crate::proto::comdex::Metadata;
use crate::state::Metadata;
use cosmwasm_std::{
    attr, entry_point, from_binary, to_binary, Addr, BankMsg, Binary, DepsMut, Env,
    IbcBasicResponse, IbcChannel, IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg,
    IbcOrder, IbcPacket, IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg,
    IbcReceiveResponse, Reply, Response, StdError, StdResult, SubMsg, SubMsgResult, WasmMsg,
};
use cw20::{Balance, Cw20ExecuteMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strategy::error::{ContractError, Never};

use crate::state::{ChannelInfo, CHANNEL_INFO};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, reply: Reply) -> Result<Response, ContractError> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
/// enforces ordering and versioning constraints
pub fn ibc_channel_open(
    _deps: DepsMut,
    _env: Env,
    msg: IbcChannelOpenMsg,
) -> Result<(), ContractError> {
    _deps
        .api
        .debug(format!("WASMDEBUG: ibc_channel_open: {:?}", msg).as_str());
    enforce_order_and_version(msg.channel(), msg.counterparty_version())?;
    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
/// record the channel in CHANNEL_INFO
pub fn ibc_channel_connect(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelConnectMsg,
) -> Result<IbcBasicResponse, ContractError> {
    deps.api
        .debug(format!("WASMDEBUG: ibc_channel_connect: {:?}", msg).as_str());
    let counterparty_version = msg.counterparty_version();
    if let Some(version) = counterparty_version {
        let p: Metadata = from_binary(&Binary::from(version.as_bytes().to_vec()))?;

        let channel: IbcChannel = msg.into();
        let info = ChannelInfo {
            id: channel.endpoint.channel_id,
            counterparty_endpoint: channel.counterparty_endpoint,
            connection_id: channel.connection_id,
            address: p.address,
        };
        CHANNEL_INFO.save(deps.storage, &info.id, &info)?;
        return Ok(IbcBasicResponse::default());
    }
    return Ok(IbcBasicResponse::default());
}

fn enforce_order_and_version(
    channel: &IbcChannel,
    counterparty_version: Option<&str>,
) -> Result<(), ContractError> {
    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_close(
    _deps: DepsMut,
    _env: Env,
    _channel: IbcChannelCloseMsg,
) -> Result<IbcBasicResponse, ContractError> {
    _deps
        .api
        .debug(format!("WASMDEBUG: ibc_channel_close: {:?}", _channel).as_str());

    Ok(IbcBasicResponse::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
/// Check to see if we have any balance here
/// We should not return an error if possible, but rather an acknowledgement of failure
pub fn ibc_packet_receive(
    deps: DepsMut,
    _env: Env,
    msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, Never> {
    deps.api
        .debug(format!("WASMDEBUG: ibc_packet_receive: {:?}", msg).as_str());

    Ok(IbcReceiveResponse::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
// check if success or failure and update balance, or return funds
pub fn ibc_packet_ack(
    deps: DepsMut,
    _env: Env,
    msg: IbcPacketAckMsg,
) -> Result<IbcBasicResponse, ContractError> {
    deps.api
        .debug(format!("WASMDEBUG: ibc_packet_ack: {:?}", msg).as_str());
    Ok(IbcBasicResponse::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
/// return fund to original sender (same as failure in ibc_packet_ack)
pub fn ibc_packet_timeout(
    deps: DepsMut,
    _env: Env,
    msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse, ContractError> {
    deps.api
        .debug(format!("WASMDEBUG: ibc_packet_timeout: {:?}", msg).as_str());

    Ok(IbcBasicResponse::new())
}

// update the balance stored on this (channel, denom) index
fn on_packet_success(deps: DepsMut, packet: IbcPacket) -> Result<IbcBasicResponse, ContractError> {
    deps.api
        .debug(format!("WASMDEBUG: on_packet_success: {:?}", packet).as_str());
    Ok(IbcBasicResponse::new())
}

fn on_packet_failure(
    deps: DepsMut,
    packet: IbcPacket,
    err: String,
) -> Result<IbcBasicResponse, ContractError> {
    deps.api
        .debug(format!("WASMDEBUG: on_packet_success: {:?}", err).as_str());
    Ok(IbcBasicResponse::new())
}
