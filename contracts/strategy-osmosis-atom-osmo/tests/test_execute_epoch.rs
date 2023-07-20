use cosmwasm_std::{coins, Uint128, StdError, OverflowError, Api, Timestamp, CosmosMsg, BankMsg};
// use cosmwasm_std::Overflow;
use cosmwasm_std::testing::{mock_env, mock_info};
use helpers::th_query;
// use osmosis_std::types::osmosis::epochs::v1beta1::EpochInfo;
use strategy_osmosis_atom_osmo::contract::{execute_update_config, execute_stake, execute_unstake, execute_epoch, execute};
use strategy_osmosis_atom_osmo::msg::{UpdateConfigMsg, QueryMsg, ExecuteEpochMsg, ExecuteMsg};
use strategy_osmosis_atom_osmo::state::{Config, EpochCallSource, Phase, CONFIG, Unbonding, UNBONDINGS};

use crate::helpers::{setup, register_ica, send_funds_to_contract, remove_free_atom_from_host_account};

mod helpers;

// test of the epoch flow
#[test]
fn epoch_deposit_phase_flow() {
    let mut deps = setup();
    let sender = "anyone";
        
    // CASE: when the step is 1 as the config is just initialized
    // without any pending deposit
    let epoch_call_source_normal = EpochCallSource::NormalEpoch;
    let res = execute_epoch(deps.as_mut(), mock_env(), epoch_call_source_normal.clone(), true, None);
    assert!(res.is_ok());

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    assert_eq!(config.phase_step, 3);

    // TODO: CASE: Step is 1, but, with pending deposit
    // take a step back to 1
    // let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    // config.phase_step = 1;
    // // set some value in to_transfer_to_host in order to test the case when there is pending deposit
    // config.host_config.free_atom_amount = Uint128::from(1000000u128);
    // config.controller_config.free_amount = Uint128::from(1000000u128);
    // CONFIG.save(deps.as_mut().storage, &config);

    // let amount = coins(
    //         10000, 
    //         config.controller_config.deposit_denom.clone()
    //     );
    // // send some funds to the contract
    // let bank_send_res = send_funds_to_contract(mock_env(),  amount);
    // assert!(bank_send_res.is_ok());
    // we cannot send funds by using this function
    // this has to be changed 

    // let res = execute_epoch(deps.as_mut(), mock_env(), epoch_call_source_normal.clone(), true, None);
    // assert!(res.is_ok());
    
    // let config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    // assert_eq!(config.phase_step, 2);

    // CASE: when the step is 3
    // first, register ica_account so that it can be executed properly
    // NOTE: This is totally random address.
    let ica_addr = String::from("osmo1aqvlxpk8dc4m2nkmxkf63a5zez9jkzgm6amkgddhfk0qj9j4rw3q662wuk");
    register_ica(deps.as_mut(), ica_addr);

    let res = execute_epoch(deps.as_mut(), mock_env(), epoch_call_source_normal.clone(), true, None);
    assert!(res.is_ok());

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    assert_eq!(config.phase_step, 4);

    // CASE: When the step is 4
    let epoch_call_source_icq = EpochCallSource::IcqCallback;
    let res = execute_epoch(deps.as_mut(), mock_env(), epoch_call_source_icq.clone(), true, None);
    assert!(res.is_ok());
    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    assert_eq!(config.phase_step, 5);

    // CASE: When the step is 5
    // And, when the contract doens't have any deposit toke to be swapped in this step of this phase
    let res = execute_epoch(deps.as_mut(), mock_env(), epoch_call_source_normal.clone(), true, None);
    assert!(res.is_ok());
    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    assert_eq!(config.phase_step, 7);

    // CASE: with pending deposit
    // take a step back to 5
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    config.phase_step = 5;
    // set some value in to_transfer_to_host in order to test the case when there is pending deposit
    config.host_config.free_atom_amount = Uint128::from(1000000u128);
    CONFIG.save(deps.as_mut().storage, &config);

    let res = execute_epoch(deps.as_mut(), mock_env(), epoch_call_source_normal.clone(), true, None);
    assert!(res.is_ok());

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    assert_eq!(config.phase_step, 6);
    // remove free_atom_amount since it's supposed to be in real execution
    remove_free_atom_from_host_account(deps.as_mut());

    // CASE: When the step is 6
    // and the callback status is success
    let epoch_call_source_ica = EpochCallSource::IcaCallback;
    let res = execute_epoch(deps.as_mut(), mock_env(), epoch_call_source_ica.clone(), true, None);
    assert!(res.is_ok());

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    assert_eq!(config.phase_step, 7);

    // CASE: When the step is 6 and the callback status is failure
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    config.phase_step = 6;
    CONFIG.save(deps.as_mut().storage, &config);
    let res = execute_epoch(deps.as_mut(), mock_env(), epoch_call_source_ica.clone(), false, None);
    assert!(res.is_ok());

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    assert_eq!(config.phase_step, 5);

    // CASE: When the step is 7
    // take a step forward to 7
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    config.phase_step = 7;
    CONFIG.save(deps.as_mut().storage, &config);

    let res = execute_epoch(deps.as_mut(), mock_env(), epoch_call_source_normal.clone(), true, None);
    assert!(res.is_ok());
    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    assert_eq!(config.phase_step, 8);

    // CASE: When the step is 8
    let res = execute_epoch(deps.as_mut(), mock_env(), epoch_call_source_icq.clone(), true, None);
    assert!(res.is_ok());
    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    assert_eq!(config.phase_step, 9);

    // CASE: When the step is 9
    // And, when the contract doens't have any deposit toke to be swapped in this step of this phase
    let res = execute_epoch(deps.as_mut(), mock_env(), epoch_call_source_normal.clone(), true, None);
    assert!(res.is_ok());
    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    assert_eq!(config.phase_step, 11);

    // CASE: When the step is 9 with some amount of free atom in host account
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    config.host_config.free_atom_amount = Uint128::from(1000000u128);
    config.phase_step = 9;
    CONFIG.save(deps.as_mut().storage, &config);

    let res = execute_epoch(deps.as_mut(), mock_env(), epoch_call_source_normal.clone(), true, None);
    assert!(res.is_ok());

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    assert_eq!(config.phase_step, 10);
    // remove free_atom_amount since it's supposed to be in real execution
    remove_free_atom_from_host_account(deps.as_mut());

    // CASE: when the step is 10
    let res = execute_epoch(deps.as_mut(), mock_env(), epoch_call_source_ica.clone(), true, None);
    assert!(res.is_ok());

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    assert_eq!(config.phase_step, 11);

    // CASE: When the step is 11
    let res = execute_epoch(deps.as_mut(), mock_env(), epoch_call_source_normal.clone(), true, None);
    assert!(res.is_ok());
    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    assert_eq!(config.phase_step, 12);

    // CASE: When the step is 12
    let res = execute_epoch(deps.as_mut(), mock_env(), epoch_call_source_icq.clone(), true, None);
    assert!(res.is_ok());
    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    assert_eq!(config.phase_step, 13);

    // CASE: when the step is 13
    // And, when the contract doens't have any deposit toke to be swapped in this step of this phase
    let res = execute_epoch(deps.as_mut(), mock_env(), epoch_call_source_normal.clone(), true, None);
    assert!(res.is_ok());
    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    assert_eq!(config.phase_step, 15);

    // CASE: when the step is 13 and there're unbondinds
    // take a step back to 13
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    config.phase_step = 13;
    CONFIG.save(deps.as_mut().storage, &config);
    // register unbonding
    let sender = deps.api.addr_validate("ununifi1j9g3qkcxm2xzfc30z462av40vx8kmwakvd00jk").unwrap();    
    let unbondings = Unbonding {
            id: 1,
            sender: sender.to_owned(),
            amount: Uint128::from(100 as u32),
            pending_start: false,
            start_time: 0,
            marked: false,
        };
    UNBONDINGS.save(deps.as_mut().storage, 1, &unbondings);

    let res = execute_epoch(deps.as_mut(), mock_env(), epoch_call_source_normal.clone(), true, None);
    assert!(res.is_ok());

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    assert_eq!(config.phase_step, 14);

    // CASE: when the step is 14 in ica_call_back
    let res = execute_epoch(deps.as_mut(), mock_env(), epoch_call_source_ica.clone(), true, None);
    assert!(res.is_ok());

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    assert_eq!(config.phase_step, 15);

    // CASE: When the step is 15
    // And when  free lp amount is 0
    let res = execute_epoch(deps.as_mut(), mock_env(), epoch_call_source_normal.clone(), true, None);
    assert!(res.is_ok());
    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    assert_eq!(config.phase, Phase::Deposit);
    assert_eq!(config.phase_step, 1);

    // CASE: When the step is 15
    // And when  free lp amount is not 0 and matured unbondings are not empty
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    config.host_config.free_lp_amount = Uint128::from(100 as u32);
    // take a step back to 15
    config.phase_step = 15;
    // change unbonding_time to useful configure for this test
    config.unbond_period = 1;
    CONFIG.save(deps.as_mut().storage, &config);
    let sender = deps.api.addr_validate("ununifi1j9g3qkcxm2xzfc30z462av40vx8kmwakvd00jk").unwrap();    
    let unbondings = Unbonding {
            id: 1,
            sender: sender.to_owned(),
            amount: Uint128::from(100 as u32),
            pending_start: false,
            start_time: 0,
            marked: false,
        };
    UNBONDINGS.save(deps.as_mut().storage, 1, &unbondings);


    let res = execute_epoch(deps.as_mut(), mock_env(), epoch_call_source_normal.clone(), true, None);
    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    assert_eq!(config.phase, Phase::Withdraw);
    assert_eq!(config.phase_step, 1);
}




