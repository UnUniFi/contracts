use crate::error::ContractError;
use crate::execute::bid::execute_bid;
use crate::execute::borrow::execute_borrow;
use crate::execute::cancel_bidding::execute_cancel_bidding;
use crate::execute::cancel_listing::execute_cancel_listing;
use crate::execute::list_nft::execute_list_nft;
use crate::execute::repay::execute_repay;
use crate::execute::update_params::execute_update_params;
use crate::msgs::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::params::query_params;
use crate::state::PARAMS;
use crate::types::Params;
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

//Initialize the contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let authority = deps.api.addr_validate(&msg.authority)?;
    let fee_collector = deps.api.addr_validate(&msg.fee_collector)?;
    let config = Params {
        authority,
        fee_collector,
        selling_fee_rate: msg.selling_fee_rate,
        interest_fee_rate: msg.interest_fee_rate,
    };
    PARAMS.save(deps.storage, &config)?;

    Ok(Response::new())
}

//Execute the handle messages.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateParams(msg) => execute_update_params(deps, env, info, msg),
        ExecuteMsg::ListNft(msg) => execute_list_nft(deps, env, info, msg),
        ExecuteMsg::CancelListing(msg) => execute_cancel_listing(deps, env, info, msg),
        ExecuteMsg::Bid(msg) => execute_bid(deps, env, info, msg),
        ExecuteMsg::CancelBidding(msg) => execute_cancel_bidding(deps, env, info, msg),
        ExecuteMsg::Borrow(msg) => execute_borrow(deps, env, info, msg),
        ExecuteMsg::Repay(msg) => execute_repay(deps, env, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_params(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
