// use crate::proto::comdex::Metadata;
use crate::error::{ContractError, Never};
use crate::msgs::ChannelInfo;
use crate::state::CHANNEL_INFO;
use crate::{
    epoch::execute_epoch,
    state::{Config, Metadata, CONFIG},
};
use cosmwasm_std::{
    entry_point, from_binary, Binary, DepsMut, Env, IbcBasicResponse, IbcChannel,
    IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg, IbcPacketAckMsg,
    IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcReceiveResponse, Reply, Response,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
        let p: Metadata = from_binary(&Binary::from(version.as_bytes().to_vec()))?;

        let channel: IbcChannel = msg.into();
        let info = ChannelInfo {
            id: channel.endpoint.channel_id,
            counterparty_endpoint: channel.counterparty_endpoint,
            connection_id: channel.connection_id.to_string(),
            address: p.address.to_string(),
        };
        CHANNEL_INFO.save(deps.storage, &info.id, &info)?;

        let mut config: Config = CONFIG.load(deps.storage)?;
        // save ica_account and channel automatically if it's the first ica_account
        if config.ica_account == "".to_string() {
            config.ica_account = p.address.to_string();
            config.ica_channel_id = info.id;
            config.ica_connection_id = channel.connection_id.to_string();
            CONFIG.save(deps.storage, &config)?;
        }
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

    Ok(IbcReceiveResponse::new())
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
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
        IcaPacketAcknowledgement::Error(_) => {
            execute_epoch(
                deps,
                env,
                crate::state::EpochCallSource::IcaCallback,
                false,
                None,
            )?;
        }
        IcaPacketAcknowledgement::Result(r) => {
            execute_epoch(
                deps,
                env,
                crate::state::EpochCallSource::IcaCallback,
                true,
                Some(r.to_vec()),
            )?;
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
