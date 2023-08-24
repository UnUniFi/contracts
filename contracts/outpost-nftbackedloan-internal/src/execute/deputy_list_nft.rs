use crate::error::ContractError;
use crate::msgs::DeputyListNftMsg;
use cosmwasm_std::CosmosMsg;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

#[cfg(not(feature = "library"))]
pub fn execute_deputy_list_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: DeputyListNftMsg,
) -> Result<Response<DeputyListNftMsg>, ContractError> {
    let mut response = Response::new();

    let deputy_list_message = DeputyListNftMsg{
        lister: msg.lister,
        class_id: msg.class_id,
        token_id: msg.token_id,
        bid_denom: msg.bid_denom,
        min_deposit_rate: msg.min_deposit_rate,
        min_bid_period: msg.min_bid_period,
    };

    response = response.add_message( CosmosMsg::Custom(deputy_list_message));

    // todo: check the tx result

    Ok(response)
}
