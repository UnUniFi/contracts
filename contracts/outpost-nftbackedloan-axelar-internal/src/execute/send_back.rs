use crate::error::ContractError;
use crate::gmp::GmpMessage;
use crate::gmp::MsgTransfer;
use crate::msgs::SendBackMsg;
use cosmwasm_std::{to_binary, CosmosMsg};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use ethabi::{encode, Token};
use proto::{cosmos::base::v1beta1::Coin, traits::MessageExt};

const TRANSFER_PORT: &str = "transfer";
const AXELAR_ADDRESS: &str = "axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5";

#[cfg(not(feature = "library"))]
pub fn execute_send_back(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: SendBackMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();

    // https://docs.axelar.dev/dev/general-message-passing/cosmos-gmp

    // TODO: verify the class id that is created by this contract via nftfactory
    // TODO: verify the ownership of the NFT of the sender
    // TODO: burn the NFT of the sender

    let origin_class_id = msg.class_id; // TODO
    let origin_token_id = msg.token_id; // TODO

    let message_payload = encode(&vec![
        Token::String(msg.destination_address.to_string()),
        Token::String(origin_class_id.to_string()),
        Token::String(origin_token_id.to_string()),
    ]);

    // {info.funds} used to pay gas. Must only contain 1 token type.
    let coin = cw_utils::one_coin(&info)?;

    let gmp_message = GmpMessage {
        destination_chain: msg.destination_chain,
        destination_address: msg.destination_address,
        payload: message_payload.to_vec(),
        type_: 1,
        fee: None,
    };

    let ibc_message = MsgTransfer {
        source_port: TRANSFER_PORT.to_string(),
        source_channel: msg.channel_to_axelar.to_string(),
        token: Some(Coin {
            denom: coin.denom,
            amount: coin.amount.to_string(),
        }),
        sender: env.contract.address.to_string(),
        receiver: AXELAR_ADDRESS.to_string(),
        timeout_height: None,
        timeout_timestamp: Some(env.block.time.plus_seconds(604_800u64).nanos()),
        memo: serde_json_wasm::to_string(&gmp_message).unwrap(),
    };
    let msg = ibc_message.to_any()?;

    response = response.add_message(CosmosMsg::Stargate {
        type_url: msg.type_url,
        value: to_binary(&msg.value)?,
    });

    Ok(response)
}
