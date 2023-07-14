use cosmwasm_std::{coins, Uint128};
use cosmwasm_std::testing::{mock_env, mock_info};
use helpers::th_query;
use strategy::error::ContractError;
use strategy::strategy::{ExecuteMsg, StakeMsg};
use strategy_osmosis_atom_osmo::contract::{execute_update_config, execute_stake};
use strategy_osmosis_atom_osmo::msg::{UpdateConfigMsg, QueryMsg};
use strategy_osmosis_atom_osmo::state::{Config};

use crate::helpers::setup;

mod helpers;

#[test]
fn stake() {
    let mut deps = setup();
    let sender = "anyone";

    // Error: because of the invalid denom
    let invalid_info = mock_info("anyone", &coins(10000 as u128, "invalid"));
    let err = execute_stake(deps.as_mut(), mock_env(), invalid_info.funds[0].clone(), invalid_info.sender).unwrap_err();
    assert_eq!(err, ContractError::NoAllowedToken {  });

    // Success: 
    let info = mock_info(sender, &coins(10000 as u128, "stake"));
    let res = execute_stake(deps.as_mut(), mock_env(), info.funds[0].clone(), info.sender).unwrap();

    assert_eq!(0, res.messages.len());

    let deposit: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    assert_eq!(Uint128::from(10000 as u64), deposit.total_deposit);
}
