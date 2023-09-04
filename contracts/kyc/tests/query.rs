use crate::helpers::setup;
use cosmwasm_std::Api;
use helpers::th_query;
use kyc::msgs::QueryMsg;
use kyc::query::params::query_params;
use kyc::state::PARAMS;
use kyc::types::Params;

mod helpers;

#[test]
fn test_query_params() {
    let mut deps = setup();

    let params = query_params(deps.as_ref()).unwrap();
    assert_eq!(params.authority.as_str(), "authority");

    let mut params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    params.authority = deps.api.addr_validate("authority2").unwrap();
    PARAMS.save(deps.as_mut().storage, &params).unwrap();

    let params = query_params(deps.as_ref()).unwrap();
    assert_eq!(params.authority.as_str(), "authority2");
}
