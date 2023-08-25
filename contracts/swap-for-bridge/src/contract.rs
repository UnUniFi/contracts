use crate::error::ContractError;
use crate::execute::deposit_liquidity::execute_deposit_liquidity;
use crate::execute::swap::execute_swap;
use crate::execute::update_config::execute_update_config;
use crate::execute::withdraw_liquidity::execute_withdraw_liquidity;
use crate::msgs::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::config::query_config;
use crate::state::CONFIG;
use crate::types::Config;
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let authority = deps.api.addr_validate(&msg.authority)?;
    let treasury = deps.api.addr_validate(&msg.treasury)?;

    let config = Config {
        authority,
        treasury,
        denoms_same_origin: msg.denoms_same_origin,
        fee: msg.fee,
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Swap(msg) => execute_swap(deps, env, info, msg),
        ExecuteMsg::DepositLiquidity(msg) => execute_deposit_liquidity(deps, env, info, msg),
        ExecuteMsg::WithdrawLiquidity(msg) => execute_withdraw_liquidity(deps, env, info, msg),
        ExecuteMsg::UpdateConfig(msg) => execute_update_config(deps, env, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}
