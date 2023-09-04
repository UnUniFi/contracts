use crate::error::ContractError;
use crate::execute::deposit_liquidity::execute_deposit_liquidity;
use crate::execute::swap::execute_swap;
use crate::execute::update_params::execute_update_params;
use crate::execute::withdraw_liquidity::execute_withdraw_liquidity;
use crate::msgs::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::params::query_params;
use crate::query::share::query_share;
use crate::state::{PARAMS, TOTAL_SHARE};
use crate::types::Params;
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
    let fee_collector = deps.api.addr_validate(&msg.fee_collector)?;

    let config = Params {
        authority,
        denoms_same_origin: msg.denoms_same_origin,
        fee_collector,
        fee_rate: msg.fee_rate,
        lp_fee_rate: msg.lp_fee_rate,
    };

    PARAMS.save(deps.storage, &config)?;
    TOTAL_SHARE.save(deps.storage, &0u128.into())?;

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
        ExecuteMsg::UpdateParams(msg) => execute_update_params(deps, env, info, msg),
        ExecuteMsg::Swap(msg) => execute_swap(deps, env, info, msg),
        ExecuteMsg::DepositLiquidity(msg) => execute_deposit_liquidity(deps, env, info, msg),
        ExecuteMsg::WithdrawLiquidity(msg) => execute_withdraw_liquidity(deps, env, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Params {} => to_binary(&query_params(deps)?),
        QueryMsg::Share { address } => to_binary(&query_share(deps, address)?),
    }
}
