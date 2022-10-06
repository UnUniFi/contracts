use crate::state::{
    Config, RewardPool, CONFIG, REWARD_POOLS, TOTAL_DEPOSITS, USER_DEPOSITS, USER_PENDING_REWARDS,
    USER_REWARD_DEBTS,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Storage,
    Uint128,
};

use yield_farming::{
    asset::{Asset, AssetInfo},
    farming::{ConfigResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
};

//Initialize the contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let config = Config {
        owner: info.sender,
        unbond_period: msg.unbond_period,
        is_freeze: false,
        default_timeout: msg.default_timeout,
        init_channel: false,
        default_remote_denom: None,
    };
    CONFIG.save(deps.storage, &config)?;
    TOTAL_DEPOSITS.save(deps.storage, &Uint128::zero())?;
    REWARD_POOLS.save(deps.storage, &vec![])?;

    Ok(Response::new())
}

//Execute the handle messages.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::UpdateConfig {
            owner,
            unbond_period,
        } => execute_update_config(deps, env, info, owner, unbond_period),
        ExecuteMsg::UpdateFreezeFlag { freeze_flag } => {
            execute_update_freeze_flag(deps, env, info, freeze_flag)
        }
        ExecuteMsg::DepositNativeToken {} => execute_deposit_native_token(deps, env, info),
        ExecuteMsg::ClaimReward { asset } => execute_claim_reward(deps, env, info, asset),
        ExecuteMsg::ClaimAllRewards {} => execute_claim_all_rewards(deps, env, info),
        ExecuteMsg::StartUnbond {} => execute_start_unbond(deps, env, info),
        ExecuteMsg::ClaimUnbond {} => execute_claim_unbond(deps, env, info),
        ExecuteMsg::SwapReward {
            source_token,
            dest_token,
        } => execute_swap_reward(deps, env, info, source_token, dest_token),
        ExecuteMsg::AutoCompoundRewards {} => execute_auto_compound_rewards(deps, env, info),
    }
}

/// Only owner can execute it. To update the owner address
pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    owner: Option<String>,
    unbond_period: Option<u64>,
) -> StdResult<Response> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    // Permission check
    if info.sender != config.owner {
        return Err(StdError::generic_err("unauthorized"));
    }
    if let Some(owner) = owner {
        config.owner = deps.api.addr_validate(&owner)?;
    }
    if let Some(unbond_period) = unbond_period {
        config.unbond_period = unbond_period;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

/// Only owner can execute it. To update the owner address
pub fn execute_update_freeze_flag(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    freeze_flag: bool,
) -> StdResult<Response> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    // Permission check
    if info.sender != config.owner {
        return Err(StdError::generic_err("unauthorized"));
    }

    config.is_freeze = freeze_flag;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_freeze_flag"))
}

pub fn execute_deposit_native_token(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> StdResult<Response> {
    let config: Config = CONFIG.load(deps.storage)?;
    if config.is_freeze {
        return Err(StdError::generic_err("contract freezed"));
    }

    let fund = info.funds[0].clone();
    if fund.amount.is_zero() {
        return Err(StdError::generic_err("invalid deposit amount"));
    }

    // let asset: Asset = Asset {
    //     info: AssetInfo::NativeToken { denom: fund.denom },
    //     amount: fund.amount,
    // };

    // swap, add liquidity, get lp tokens and deposit lp tokens into the target farming pool
    update_pool(deps.storage, env)?;
    withdraw_reward(deps.storage, info.sender.to_string())?;
    // USER_DEPOSITS, TOTAL_DEPOSITS update
    update_user_reward_debt(deps.storage, info.sender.to_string())?;

    Ok(Response::new().add_attribute("action", "deposit_native_token"))
}

pub fn execute_claim_reward(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _asset: Asset,
) -> StdResult<Response> {
    Ok(Response::new().add_attribute("action", "execute_claim_reward"))
}

pub fn execute_claim_all_rewards(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> StdResult<Response> {
    Ok(Response::new().add_attribute("action", "execute_claim_all_rewards"))
}

pub fn execute_start_unbond(_deps: DepsMut, _env: Env, _info: MessageInfo) -> StdResult<Response> {
    Ok(Response::new().add_attribute("action", "execute_start_unbond"))
}

pub fn execute_claim_unbond(_deps: DepsMut, _env: Env, _info: MessageInfo) -> StdResult<Response> {
    Ok(Response::new().add_attribute("action", "execute_claim_unbond"))
}

pub fn execute_swap_reward(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _source_token: AssetInfo,
    _dest_token: AssetInfo,
) -> StdResult<Response> {
    Ok(Response::new().add_attribute("action", "execute_swap_reward"))
}

pub fn execute_auto_compound_rewards(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> StdResult<Response> {
    Ok(Response::new().add_attribute("action", "execute_auto_compound_rewards"))
}

pub fn update_pool(store: &mut dyn Storage, env: Env) -> StdResult<()> {
    // claim reward from the osmosis pool
    let reward_pools = REWARD_POOLS.load(store)?;
    let mut new_reward_pools: Vec<RewardPool> = vec![];
    let total_deposits = TOTAL_DEPOSITS.load(store)?;

    for mut reward_pool in reward_pools {
        if total_deposits.is_zero() {
            reward_pool.acc_reward_per_share = Uint128::from(env.block.time.seconds());
        } else {
            let new_rewards = Uint128::zero(); // claimed reward of selected pool
            reward_pool.acc_reward_per_share += new_rewards
                .checked_mul(Uint128::from(1_000_000_000_000u128))?
                .checked_div(total_deposits)?;
        }
        new_reward_pools.push(reward_pool);
    }

    REWARD_POOLS.save(store, &new_reward_pools)?;
    Ok(())
}

pub fn withdraw_reward(store: &mut dyn Storage, wallet: String) -> StdResult<()> {
    let reward_pools = REWARD_POOLS.load(store)?;

    for reward_pool in reward_pools {
        if let Some(user_deposit) = USER_DEPOSITS.may_load(store, wallet.clone())? {
            let key = reward_pool.reward_token.to_string() + &wallet;
            if let Some(user_reward_debt) = USER_REWARD_DEBTS.may_load(store, key.clone())? {
                let pending = user_deposit
                    .checked_mul(reward_pool.acc_reward_per_share)?
                    .checked_div(Uint128::from(1_000_000_000_000u128))?
                    .checked_sub(user_reward_debt)?;
                if !pending.is_zero() {
                    USER_PENDING_REWARDS.save(store, key, &pending)?;
                }
            }
        }
    }

    Ok(())
}

pub fn update_user_reward_debt(store: &mut dyn Storage, wallet: String) -> StdResult<()> {
    let reward_pools = REWARD_POOLS.load(store)?;

    for reward_pool in reward_pools {
        if let Some(user_deposit) = USER_DEPOSITS.may_load(store, wallet.clone())? {
            let key = reward_pool.reward_token.to_string() + &wallet;
            let user_reward_debt = user_deposit
                .checked_mul(reward_pool.acc_reward_per_share)?
                .checked_div(Uint128::from(1_000_000_000_000u128))?;
            USER_REWARD_DEBTS.save(store, key, &user_reward_debt)?;
        }
    }

    Ok(())
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
        unbond_period: config.unbond_period,
        is_freeze: config.is_freeze,
        default_timeout: config.default_timeout,
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
