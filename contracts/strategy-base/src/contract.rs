use crate::state::{Config, LockInfo, UnlockInfo, CONFIG};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, coin, to_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, IbcMsg,
    IbcQuery, MessageInfo, Order, PortIdResponse, Response, StdResult, Storage, Uint128, WasmMsg,
};

use cw_utils::{nonpayable, one_coin};
use strategy::{
    error::ContractError,
    strategy::{
        ClaimAllTokensMsg, ClaimTokensMsg, ConfigResponse, CreateLockupMsg, DepositMsg, ExecuteMsg,
        ExitPoolMsg, InstantiateMsg, JoinPoolMsg, LockTokensMsg, MigrateMsg, QueryMsg, SwapMsg,
        TransferMsg, UnlockTokensMsg,
    },
};

//Initialize the contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        owner: info.sender,
        unlock_period: msg.unlock_period,
        is_freeze: false,
        default_timeout: msg.default_timeout,
        init_channel: false,
        default_remote_denom: None,
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
            unlock_period,
        } => execute_update_config(deps, env, info, owner, unlock_period),
        ExecuteMsg::Deposit(msg) => {
            let coin = one_coin(&info)?;
            execute_deposit(deps, env, msg, coin.amount, info.sender)
        }
    }
}

/// Only owner can execute it.
pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    owner: Option<String>,
    unlock_period: Option<u64>,
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    // Permission check
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }
    if let Some(owner) = owner {
        config.owner = deps.api.addr_validate(&owner)?;
    }
    if let Some(unlock_period) = unlock_period {
        config.unlock_period = unlock_period;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

pub fn execute_deposit(
    deps: DepsMut,
    env: Env,
    msg: DepositMsg,
    amount: Uint128,
    sender: Addr,
) -> Result<Response, ContractError> {
    panic!("not implemented!")
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config: Config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: config.owner.to_string(),
        unlock_period: config.unlock_period,
        is_freeze: config.is_freeze,
        default_timeout: config.default_timeout,
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_helpers::*;

    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, StdError};

    #[test]
    fn proper_deposit() {}
}
