use crate::msg::{ChannelResponse, FeeInfo, ListChannelsResponse};
use crate::state::{
    Config, Unbonding, CHANNEL_INFO, CONFIG, DEPOSITS, HOST_LP_RATE_MULTIPLIER,
    STAKE_RATE_MULTIPLIER, UNBONDINGS,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    Addr, BalanceResponse, BankQuery, Decimal, Deps, Order, QuerierWrapper, QueryRequest,
    StdResult, Storage, Uint128,
};

pub fn query_balance(
    querier: &QuerierWrapper,
    account_addr: Addr,
    denom: String,
) -> StdResult<Uint128> {
    let balance: BalanceResponse = querier.query(&QueryRequest::Bank(BankQuery::Balance {
        address: account_addr.to_string(),
        denom,
    }))?;
    Ok(balance.amount.amount)
}

pub fn query_config(deps: Deps) -> StdResult<Config> {
    let config: Config = CONFIG.load(deps.storage)?;
    Ok(config)
}

pub fn query_list_channels(deps: Deps) -> StdResult<ListChannelsResponse> {
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

pub fn query_unbonding(deps: Deps, addr: String) -> StdResult<Uint128> {
    let config: Config = CONFIG.load(deps.storage)?;
    let mut pending_unbonding_lp = Uint128::new(0u128);
    let unbondings = query_unbondings(deps.storage, Some(DEFAULT_LIMIT))?;
    for unbonding in unbondings {
        if unbonding.sender == addr {
            pending_unbonding_lp += unbonding.amount;
        }
    }
    Ok(pending_unbonding_lp * config.host_config.lp_redemption_rate / HOST_LP_RATE_MULTIPLIER)
}

pub fn query_bonded(deps: Deps, addr: String) -> StdResult<Uint128> {
    let config: Config = CONFIG.load(deps.storage)?;
    let deposit = DEPOSITS.load(deps.storage, addr)?;
    Ok(deposit.amount * config.redemption_rate / STAKE_RATE_MULTIPLIER)
}

pub const DEFAULT_LIMIT: u32 = 50;
pub fn query_unbondings(storage: &dyn Storage, limit: Option<u32>) -> StdResult<Vec<Unbonding>> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT) as usize;

    UNBONDINGS
        .range(storage, None, None, Order::Ascending)
        .take(limit)
        .map(|item| {
            let (_, v) = item?;
            Ok(v)
        })
        .collect::<StdResult<Vec<Unbonding>>>()
}
