use crate::msg::{
    ChannelResponse, ExecuteMsg, FeeInfo, InstantiateMsg, ListChannelsResponse, MigrateMsg,
    QueryMsg,
};
use crate::state::{Config, DepositInfo, CHANNEL_INFO, CONFIG, DEPOSITS};
// use proto::cosmos::base::v1beta1::Coin as ProtoCoin;
// use proto::cosmos::staking::v1beta1::MsgDelegate;
// use cosmrs::proto::cosmos::staking::v1beta1::MsgDelegate;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, to_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Decimal, Deps, DepsMut, Env, IbcMsg,
    IbcTimeout, MessageInfo, Order, Response, StdResult, Timestamp, Uint128,
};
use cw_utils::one_coin;
// use prost::Message;
use strategy::error::ContractError;

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
        ExecuteMsg::IcaAddLiquidity(msg) => {
            execute_ica_add_liquidity(deps, msg.channel_id, msg.timeout)
        }
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

pub fn execute_stake(deps: DepsMut, coin: Coin, sender: Addr) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if config.deposit_denom != coin.denom {
        return Err(ContractError::NoAllowedToken {});
    }
    let amount = coin.amount;
    let redemption_rate_multiplier = Uint128::from(1000000u128);
    let redemption_rate = config.redemption_rate;
    DEPOSITS.update(
        deps.storage,
        sender.to_string(),
        |deposit: Option<DepositInfo>| -> StdResult<_> {
            if let Some(unwrapped) = deposit {
                let stake_amount = amount * redemption_rate_multiplier / redemption_rate;
                return Ok(DepositInfo {
                    sender: sender.clone(),
                    amount: unwrapped.amount.checked_add(stake_amount)?,
                });
            }
            Ok(DepositInfo {
                sender: sender.clone(),
                amount: amount,
            })
        },
    )?;
    config.total_deposit += amount;
    CONFIG.save(deps.storage, &config)?;

    let rsp = Response::default()
        .add_attribute("action", "stake")
        .add_attribute("sender", sender)
        .add_attribute("amount", amount);
    Ok(rsp)
}

pub fn execute_unstake(
    deps: DepsMut,
    amount: Uint128,
    sender: Addr,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
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

    config.total_deposit -= amount;
    CONFIG.save(deps.storage, &config)?;
    let bank_send_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: sender.to_string(),
        amount: coins(amount.u128(), &config.deposit_denom),
    });
    let rsp = Response::new()
        .add_message(bank_send_msg)
        .add_attribute("action", "unstake")
        .add_attribute("sender", sender)
        .add_attribute("amount", amount);
    Ok(rsp)
}

pub fn execute_add_rewards(deps: DepsMut, coin: Coin) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if config.deposit_denom != coin.denom {
        return Err(ContractError::NoAllowedToken {});
    }
    let amount = coin.amount;
    let old_total_deposit = config.total_deposit;
    config.total_deposit += amount;
    config.redemption_rate = config.redemption_rate * config.total_deposit / old_total_deposit;
    CONFIG.save(deps.storage, &config)?;
    let rsp = Response::default()
        .add_attribute("action", "add_rewards")
        .add_attribute("amount", amount);
    Ok(rsp)
}

// TODO: add endpoint for ibc transfer initiated by yieldaggregator module endblocker
// TODO: add endpoint for initiating stake, unstake, claim rewards + autocompound for each epoch yieldaggregator trigger

pub fn execute_ica_add_liquidity(
    deps: DepsMut,
    channel_id: String,
    timeout: u64,
) -> Result<Response, ContractError> {
    // let ibc_packet = MsgDeposit {
    //     /// depositor specifies the bech32-encoded address that makes a deposit to the pool
    //     depositor: "".to_string(),
    //     /// pool_id specifies the pool id
    //     pool_id: 1,
    //     /// deposit_coins specifies the amount of coins to deposit.
    //     deposit_coins: vec![coins(1000u128, "uatom")],
    //     app_id: 1,
    // };
    // let ibc_packet = MsgDelegate {
    //     delegator_address: "".to_string(),
    //     validator_address: "".to_string(),
    //     amount: Some(ProtoCoin {
    //         denom: "uatom".to_string(),
    //         amount: "1".to_string(),
    //     }),
    // };
    // let mut buf = vec![];
    // ibc_packet.encode(&mut buf).unwrap();

    // let decoded: MsgDeposit = Message::decode(&buf[..]).unwrap();
    // println!("serialized: {:?}\noriginal: {:?}", buf, decoded)

    let timestamp = Timestamp::from_seconds(timeout);
    // let ibc_msg = IbcMsg::SendPacket {
    //     channel_id: channel_id,
    //     data: to_binary(&buf[..])?,
    //     timeout: IbcTimeout::from(timestamp),
    // };

    let res = Response::new()
        // .add_message(ibc_msg)
        .add_attribute("action", "ica_add_liquidity");
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::Unbonding { addr } => to_binary(&query_unbonding(deps, addr)?),
        QueryMsg::Bonded { addr } => to_binary(&query_bonded(deps, addr)?),
        QueryMsg::Fee {} => to_binary(&query_fee_info(deps)?),
        QueryMsg::ListChannels {} => to_binary(&query_list_channels(deps)?),
        QueryMsg::Channel { id } => to_binary(&query_channel(deps, id)?),
    }
}

pub fn query_config(deps: Deps) -> StdResult<Config> {
    let config: Config = CONFIG.load(deps.storage)?;
    Ok(config)
}

fn query_list_channels(deps: Deps) -> StdResult<ListChannelsResponse> {
    let channels = CHANNEL_INFO
        .range_raw(deps.storage, None, None, Order::Ascending)
        .map(|r| r.map(|(_, v)| v))
        .collect::<StdResult<_>>()?;
    Ok(ListChannelsResponse { channels: channels })
}

// make public for ibc tests
pub fn query_channel(deps: Deps, id: String) -> StdResult<ChannelResponse> {
    let info = CHANNEL_INFO.load(deps.storage, &id)?;
    Ok(ChannelResponse { info })
}

pub fn query_fee_info(_: Deps) -> StdResult<FeeInfo> {
    Ok(FeeInfo {
        deposit_fee_rate: Decimal::zero(),
        withdraw_fee_rate: Decimal::zero(),
        interest_fee_rate: Decimal::zero(),
    })
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
