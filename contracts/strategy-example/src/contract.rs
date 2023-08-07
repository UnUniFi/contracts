use crate::error::ContractError;
use crate::execute::add_rewards::execute_add_rewards;
use crate::execute::stake::execute_stake;
use crate::execute::unstake::execute_unstake;
use crate::execute::update_config::execute_update_config;
use crate::msgs::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::bonded::query_bonded;
use crate::query::config::query_config;
use crate::query::fee::query_fee;
use crate::query::unbonding::query_unbonding;
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
    let redemption_rate_multiplier = Uint128::from(1000000u128);
    let config = Config {
        owner: info.sender,
        unbond_period: msg.unbond_period,
        deposit_denom: msg.deposit_denom,
        redemption_rate: redemption_rate_multiplier,
        total_deposit: Uint128::from(0u128),
    };
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
        ExecuteMsg::UpdateConfig {
            owner,
            unbond_period,
            deposit_denom,
        } => execute_update_config(deps, env, info, owner, unbond_period, deposit_denom),
        ExecuteMsg::Stake(_) => {
            let coin: Coin = one_coin(&info)?;
            execute_stake(deps, coin, info.sender)
        }
        ExecuteMsg::Unstake(msg) => execute_unstake(deps, msg.amount, info.sender),
        ExecuteMsg::AddRewards(_) => {
            let coin = one_coin(&info)?;
            execute_add_rewards(deps, coin)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::Unbonding { addr } => to_binary(&query_unbonding(deps, addr)?),
        QueryMsg::Bonded { addr } => to_binary(&query_bonded(deps, addr)?),
        QueryMsg::Fee {} => to_binary(&query_fee(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
