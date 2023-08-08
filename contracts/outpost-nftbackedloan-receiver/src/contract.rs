use crate::error::ContractError;
use crate::execute::borrow::execute_borrow;
use crate::execute::end_listing::execute_end_listing;
use crate::execute::list_nft::execute_list_nft;
use crate::execute::repay::execute_repay;
use crate::execute::update_config::execute_update_config;
use crate::execute::withdraw_nft::execute_withdraw_nft;
use crate::msgs::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::config::query_config;
use crate::state::CONFIG;
use crate::types::Config;
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw_utils::one_coin;

//Initialize the contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config { owner: info.sender };
    CONFIG.save(deps.storage, &config)?;

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
        ExecuteMsg::UpdateConfig { owner } => execute_update_config(deps, env, info, owner),
        ExecuteMsg::ListNft {
            sender,
            source_chain,
            class_id,
            token_id,
        } => execute_list_nft(deps, env, info),
        ExecuteMsg::Borrow {
            sender,
            source_chain,
            class_id,
            token_id,
            amount,
        } => execute_borrow(deps, env, info),
        ExecuteMsg::Repay {
            source_chain,
            class_id,
            token_id,
            amount,
        } => execute_repay(deps, env, info),
        ExecuteMsg::EndListing {
            sender,
            source_chain,
            class_id,
            token_id,
        } => execute_end_listing(deps, env, info),
        ExecuteMsg::WithdrawNft {
            sender,
            source_chain,
            class_id,
            token_id,
        } => execute_withdraw_nft(deps, env, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
