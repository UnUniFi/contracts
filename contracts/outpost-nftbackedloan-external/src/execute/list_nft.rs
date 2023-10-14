use crate::error::ContractError;
use crate::msgs::ListNftMsg;
use cosmwasm_std::to_binary;
use cosmwasm_std::CosmosMsg;
use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response};

#[cfg(not(feature = "library"))]
pub fn execute_list_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ListNftMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();

    // before this contract nft approve on frontend
    response = response.add_message(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: msg.cw721_contract,
        msg: to_binary(&cw721::Cw721ExecuteMsg::TransferNft {
            recipient: env.contract.address.to_string(),
            token_id: msg.token_id,
        })?,
        funds: vec![],
    }));

    // TODO: send IBC packet to the internal contract
    // Ref: https://github.com/cosmos/ibc-apps/tree/main/modules/ibc-hooks

    Ok(response)
}
