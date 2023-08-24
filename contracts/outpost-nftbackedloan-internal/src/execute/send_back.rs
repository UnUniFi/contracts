use crate::error::ContractError;
use crate::msgs::SendBackMsg;
use cosmwasm_std::to_binary;
use cosmwasm_std::CosmosMsg;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

#[cfg(not(feature = "library"))]
pub fn execute_send_back(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: SendBackMsg,
) -> Result<Response, ContractError> {

    let mut response = Response::new();

    //  send IBC packet to the external contract
    response = response.add_message(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: msg.origin_address,
        msg: to_binary(&cw721::Cw721ExecuteMsg::TransferNft {
            recipient: env.contract.address.to_string(),
            token_id: msg.token_id,
        })?,
        funds: vec![],
    }));

    Ok(response)
}
