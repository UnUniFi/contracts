use crate::state::{Config, DepositInfo, CONFIG, DEPOSITS};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, coin, coins, to_binary, Addr, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, Response, StdResult, Uint128,
};
use cw_utils::{nonpayable, one_coin};
use strategy::{
    error::ContractError,
    strategy::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, StakeMsg, UnstakeMsg},
};

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
            let coin = one_coin(&info)?;
            execute_stake(deps, coin.amount, info.sender)
        }
        ExecuteMsg::Unstake(msg) => execute_unstake(deps, msg.amount, info.sender),
        // TODO: add a way to update redemption rate automatically or manually by sending tokens
    }
}

/// Only owner can execute it.
pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    owner: Option<String>,
    unbond_period: Option<u64>,
    deposit_denom: Option<String>,
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    // Permission check
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(owner) = owner {
        config.owner = deps.api.addr_validate(&owner)?;
    }
    if let Some(unbond_period) = unbond_period {
        config.unbond_period = unbond_period;
    }
    if let Some(deposit_denom) = deposit_denom {
        config.deposit_denom = deposit_denom;
    }

    CONFIG.save(deps.storage, &config)?;
    let resp = Response::new()
        .add_attribute("action", "update_config")
        .add_attribute("owner", config.owner.to_string())
        .add_attribute("unbond_period", config.unbond_period.to_string())
        .add_attribute("deposit_denom", config.deposit_denom.to_string());

    Ok(resp)
}

pub fn execute_stake(
    deps: DepsMut,
    amount: Uint128,
    sender: Addr,
) -> Result<Response, ContractError> {
    let redemption_rate_multiplier = Uint128::from(1000000u128);
    let config = CONFIG.load(deps.storage)?;
    DEPOSITS.update(
        deps.storage,
        sender.to_string(),
        |deposit: Option<DepositInfo>| -> StdResult<_> {
            if let Some(unwrapped) = deposit {
                let stake_amount = amount * redemption_rate_multiplier / config.redemption_rate;
                return Ok(DepositInfo {
                    sender: sender,
                    amount: unwrapped.amount.checked_add(stake_amount)?,
                });
            }
            Ok(DepositInfo {
                sender: sender,
                amount: amount,
            })
        },
    )?;
    let rsp = Response::default();
    // TODO: event emission
    Ok(rsp)
}

pub fn execute_unstake(
    deps: DepsMut,
    amount: Uint128,
    sender: Addr,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let redemption_rate_multiplier = Uint128::from(1000000u128);
    DEPOSITS.update(
        deps.storage,
        sender.to_string(),
        |deposit: Option<DepositInfo>| -> StdResult<_> {
            if let Some(unwrapped) = deposit {
                let unstake_amount = amount * redemption_rate_multiplier / config.redemption_rate;
                return Ok(DepositInfo {
                    sender: sender.clone(),
                    amount: unwrapped.amount.checked_sub(unstake_amount)?,
                });
            }
            Ok(DepositInfo {
                sender: sender.clone(),
                amount: amount,
            })
        },
    )?;

    let bank_send_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: sender.to_string(),
        amount: coins(amount.u128(), &config.deposit_denom),
    });
    let rsp = Response::new().add_message(bank_send_msg);
    // TODO: event emission
    Ok(rsp)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::Unbonding { addr } => to_binary(&query_unbonding(deps, addr)?),
        QueryMsg::Bonded { addr } => to_binary(&query_bonded(deps, addr)?),
    }
}

pub fn query_config(deps: Deps) -> StdResult<Config> {
    let config: Config = CONFIG.load(deps.storage)?;
    Ok(config)
}

pub fn query_unbonding(_: Deps, _: String) -> StdResult<Uint128> {
    Ok(Uint128::from(0u128))
}

pub fn query_bonded(deps: Deps, addr: String) -> StdResult<Uint128> {
    let config: Config = CONFIG.load(deps.storage)?;
    let deposit = DEPOSITS.load(deps.storage, addr)?;
    let redemption_rate_multiplier = Uint128::from(1000000u128);
    Ok(deposit.amount * config.redemption_rate / redemption_rate_multiplier)
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
    fn execute_update_config() {}

    #[test]
    fn execute_stake() {}

    #[test]
    fn execute_unstake() {}

    #[test]
    fn query_unbonding() {}

    #[test]
    fn query_bonded() {}
}
