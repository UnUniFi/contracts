use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{coins, Uint128};
use helpers::th_query;
use strategy::msgs::{ExecuteMsg, StakeMsg};
use strategy_osmosis::contract::{execute_stake, execute_update_config};
use strategy_osmosis::error::ContractError;
use strategy_osmosis::state::Config;
use strategy_osmosis_interface::strategy::{QueryMsg, UpdateConfigMsg};

use crate::helpers::setup;

mod helpers;

#[test]
fn stake() {
    let mut deps = setup();
    let sender = "anyone";

    // Error: because of the invalid denom
    let invalid_info = mock_info("anyone", &coins(10000 as u128, "invalid"));
    let err = execute_stake(
        deps.as_mut(),
        mock_env(),
        invalid_info.funds[0].clone(),
        invalid_info.sender,
    )
    .unwrap_err();
    assert_eq!(err, ContractError::NoAllowedToken {});

    // Success:
    let info = mock_info(sender, &coins(10000 as u128, "stake"));
    let res = execute_stake(
        deps.as_mut(),
        mock_env(),
        info.funds[0].clone(),
        info.sender,
    )
    .unwrap();

    assert_eq!(0, res.messages.len());

    let deposit: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(Uint128::from(10000 as u64), deposit.total_deposit);
}
