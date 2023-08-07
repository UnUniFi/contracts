use cosmwasm_std::{to_binary, Env, IbcMsg, IbcTimeout, Response, StdError};
use prost::Message;
use prost_types::Any;
use proto::ibc::applications::interchain_accounts::v1::CosmosTx;
use serde::{Deserialize, Serialize};
use ununifi_binding::v0::binding::UnunifiMsg;

/// Maximum length of address
pub const MAX_ADDR_LEN: usize = 255;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct InterchainAccountPacketData {
    pub r#type: i32,
    pub data: Vec<u8>,
    pub memo: String,
}

pub fn send_ica_tx(
    env: Env,
    ica_channel_id: String,
    transfer_timeout: u64,
    action: String,
    msgs: Vec<Any>,
) -> Result<Response<UnunifiMsg>, StdError> {
    let cosmos_tx = CosmosTx { messages: msgs };
    let mut cosmos_tx_buf = vec![];
    cosmos_tx.encode(&mut cosmos_tx_buf).unwrap();

    let ibc_packet = InterchainAccountPacketData {
        r#type: 1,
        data: cosmos_tx_buf,
        memo: action.to_string(),
    };

    let timeout = env.block.time.plus_seconds(transfer_timeout);
    let ibc_msg = IbcMsg::SendPacket {
        channel_id: ica_channel_id,
        data: to_binary(&ibc_packet)?,
        timeout: IbcTimeout::from(timeout),
    };

    let res = Response::new()
        .add_message(ibc_msg)
        .add_attribute("action", action.to_string());
    return Ok(res);
}
