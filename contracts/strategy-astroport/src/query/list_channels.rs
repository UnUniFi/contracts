use crate::msgs::ListChannelsResponse;
use crate::state::CHANNEL_INFO;
use cosmwasm_std::{Deps, Order, StdResult};

pub fn query_list_channels(deps: Deps) -> StdResult<ListChannelsResponse> {
    let channels = CHANNEL_INFO
        .range_raw(deps.storage, None, None, Order::Ascending)
        .map(|r| r.map(|(_, v)| v))
        .collect::<StdResult<_>>()?;
    Ok(ListChannelsResponse { channels: channels })
}
