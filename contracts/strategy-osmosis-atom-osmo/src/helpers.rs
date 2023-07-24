use crate::binding::AddressBytes;
use crate::state::InterchainAccountPacketData;
use crate::{
    binding::UnunifiMsg,
    state::{Config, CONFIG},
};
use cosmwasm_std::{to_binary, Env, IbcMsg, IbcTimeout, Response, Storage};
use prost::Message;
use prost_types::Any;
use proto::ibc::applications::interchain_accounts::v1::CosmosTx;
use strategy::error::ContractError;

/// Maximum length of address
pub const MAX_ADDR_LEN: usize = 255;

pub fn send_ica_tx(
    store: &dyn Storage,
    env: Env,
    action: String,
    msgs: Vec<Any>,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config: Config = CONFIG.load(store)?;
    let cosmos_tx = CosmosTx { messages: msgs };
    let mut cosmos_tx_buf = vec![];
    cosmos_tx.encode(&mut cosmos_tx_buf).unwrap();

    let ibc_packet = InterchainAccountPacketData {
        r#type: 1,
        data: cosmos_tx_buf,
        memo: action.to_string(),
    };

    let timeout = env.block.time.plus_seconds(config.transfer_timeout);
    let ibc_msg = IbcMsg::SendPacket {
        channel_id: config.ica_channel_id,
        data: to_binary(&ibc_packet)?,
        timeout: IbcTimeout::from(timeout),
    };

    let res = Response::new()
        .add_message(ibc_msg)
        .add_attribute("action", action.to_string());
    return Ok(res);
}

/// Decodes a bech32 encoded string and converts to base64 encoded bytes
/// https://github.com/cosmos/cosmos-sdk/blob/ad9e5620fb3445c716e9de45cfcdb56e8f1745bf/types/bech32/bech32.go#L20
pub fn decode_and_convert(encoded: &str) -> Result<AddressBytes, ContractError> {
    let (_hrp, bytes, _variant) = bech32::decode(encoded)?;

    Ok(bech32::convert_bits(&bytes, 5, 8, false)?)
}

/// Prefixes the address bytes with its length
pub fn length_prefix<AddrBytes: AsRef<[u8]>>(addr: AddrBytes) -> Result<Vec<u8>, ContractError> {
    let bz_length = addr.as_ref().len();

    if bz_length == 0 {
        return Ok(vec![]);
    }

    if bz_length > MAX_ADDR_LEN {
        return Err(ContractError::MaxAddrLength {});
    }

    let mut p: Vec<u8> = vec![bz_length as u8];
    p.extend_from_slice(addr.as_ref());

    Ok(p)
}
