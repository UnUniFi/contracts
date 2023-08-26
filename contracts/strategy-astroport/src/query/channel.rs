use crate::msgs::ChannelResponse;
use crate::state::CHANNEL_INFO;
use cosmwasm_std::{Deps, StdResult};

// make public for ibc tests
pub fn query_channel(deps: Deps, id: String) -> StdResult<ChannelResponse> {
    let info = CHANNEL_INFO.load(deps.storage, &id)?;
    Ok(ChannelResponse { info })
}
