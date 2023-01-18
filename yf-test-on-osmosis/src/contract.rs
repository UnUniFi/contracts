#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::{execute_exit_swap_share, execute_join_swap_extern, handle_join_swap_reply};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::query_depositor_share_amount;
use crate::state::SWAP_REPLY_STATES;
// use crate::state::{State, STATE, SWAP_REPLY_STATES};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:yf-test-on-osmosis";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Msg Reply IDs
pub const JOIN_SWAP_REPLY_ID: u64 = 1u64;
pub const EXIT_SWAP_REPLY_ID: u64 = 2u64;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _: InstantiateMsg,
) -> Result<Response, ContractError> {
    // set contract version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // return OK
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::JoinSwapExtern {
            pool_id,
            token_in,
            share_out_min_amount,
        } => execute_join_swap_extern(deps, env, info, pool_id, token_in, share_out_min_amount),
        ExecuteMsg::ExitSwapShare {
            pool_id,
            token_out_denom,
            share_in_amount,
            token_out_min_amount,
        } => execute_exit_swap_share(
            deps,
            env,
            info,
            pool_id,
            token_out_denom,
            share_in_amount,
            token_out_min_amount,
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::DepositorShareAmount { depositor } => {
            to_binary(&query_depositor_share_amount(deps, &depositor)?)
        }
    }
}

// TODO: implement actual logic for both join_swap and exit_swap reply cases
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    if msg.id == JOIN_SWAP_REPLY_ID {
        // get intermediate swap reply state. Error if not found.
        let swap_msg_reply_state = SWAP_REPLY_STATES.load(deps.storage, msg.id)?;

        // prune intermedate state since it's no longer necessary
        SWAP_REPLY_STATES.remove(deps.storage, msg.id);

        handle_join_swap_reply(deps, msg, swap_msg_reply_state)
    }
    // else if msg.id == EXIT_SWAP_REPLY_ID {
    // handle_exit_swap_reply(deps, msg)
    // }
    else {
        Ok(Response::new())
    }
}
