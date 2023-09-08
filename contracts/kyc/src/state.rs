use crate::types::{InformationRequest, Params, Provider, Verification};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const PARAMS: Item<Params> = Item::new("params");

pub const PROVIDER_ID: Item<ProviderId> = Item::new("provider_id");
pub const PROVIDERS: Map<ProviderId, Provider> = Map::new("providers");

pub const VERIFICATIONS: Map<(Addr, ProviderId), Verification> = Map::new("verifications");

pub const INFORMATION_REQUEST_ID: Item<InformationRequestId> = Item::new("information_request_id");
pub const INFORMATION_REQUESTS: Map<(Addr, InformationRequestId), InformationRequest> =
    Map::new("information_requests");

pub type ProviderId = u64;
pub type InformationRequestId = u64;
