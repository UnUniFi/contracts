use cosmwasm_std::{Deps, StdResult};
use strategy::v1::msgs::KycResp;

pub fn query_kyc_info(_: Deps) -> StdResult<KycResp> {
    Ok(KycResp {
        kyc_required: false,
        trusted_provider_ids: vec![],
    })
}
