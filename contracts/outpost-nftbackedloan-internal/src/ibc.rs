use crate::error::ContractError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    entry_point, from_binary, Binary, DepsMut, Env, IbcBasicResponse, IbcChannel,
    IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg, IbcPacketAckMsg,
    IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcReceiveResponse, Never, Reply, Response,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _reply: Reply) -> Result<Response, ContractError> {
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
        return Ok(IbcBasicResponse::default());
    }

    return Ok(IbcBasicResponse::default());
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

    // TODO: Mint NFT via nftfactory
    // TODO: Use new binding function "ListNftFromDeputyContract"

    Ok(IbcReceiveResponse::new())
}

#[cw_serde]
pub enum IcaPacketAcknowledgement {
    Result(Binary),
    Error(String),
}

#[cfg_attr(not(feature = "library"), entry_point)]
// check if success or failure and update balance, or return funds
pub fn ibc_packet_ack(
    deps: DepsMut,
    env: Env,
    msg: IbcPacketAckMsg,
) -> Result<IbcBasicResponse, ContractError> {
    deps.api
        .debug(format!("WASMDEBUG: ibc_packet_ack: {:?}", msg).as_str());

    let ack: IcaPacketAcknowledgement = from_binary(&msg.acknowledgement.data)
        .unwrap_or_else(|_| IcaPacketAcknowledgement::Error(msg.acknowledgement.data.to_base64()));
    match ack {
        IcaPacketAcknowledgement::Error(_) => {}
        IcaPacketAcknowledgement::Result(r) => {
            // TODO: Burn Voucher Token
        }
    }
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
