use crate::error::ContractError;
use crate::msgs::DeputyListNftMsg;
use cosmwasm_std::CosmosMsg;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use ununifi_binding::v1::binding::UnunifiMsg;

#[cfg(not(feature = "library"))]
pub fn execute_deputy_list_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: DeputyListNftMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut response = Response::new();
    // class_id = chain + contract_addr
    // token_id = original_token_id

    // mint nft (by contract) & send nft (contract to lister) + list nft (by lister)
    let deputy_list_message = UnunifiMsg::DeputyListNft{
        lister: msg.lister,
        class_id: msg.class_id,
        token_id: msg.token_id,
        bid_denom: msg.bid_denom,
        min_deposit_rate: msg.min_deposit_rate,
        min_bid_period: msg.min_bid_period,
    };

    response = response.add_message( CosmosMsg::Custom(deputy_list_message));

    Ok(response)
}
