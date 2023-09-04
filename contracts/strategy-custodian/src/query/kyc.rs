use crate::state::PARAMS;
use crate::types::Params;
use cosmwasm_std::{Deps, StdResult};
use strategy::v1::msgs::KycResp;

#[cfg(not(feature = "library"))]
pub fn query_kyc(deps: Deps) -> StdResult<KycResp> {
    let params: Params = PARAMS.load(deps.storage)?;
    Ok(KycResp {
        kyc_required: false,
        trusted_provider_ids: params.trusted_kyc_provider_ids,
    })
}
