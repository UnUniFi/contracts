use cosmwasm_std::{
    coins, Api, BankMsg, CosmosMsg, DepsMut, OverflowError, StdError, Timestamp, Uint128,
};
// use cosmwasm_std::Overflow;
use cosmwasm_std::testing::{mock_env, mock_info};
use helpers::th_query;
// use osmosis_std::types::osmosis::epochs::v1beta1::EpochInfo;
use strategy_osmosis::contract::{execute, execute_stake, execute_unstake, execute_update_config};
use strategy_osmosis::epoch::execute_epoch;
use strategy_osmosis::state::{Config, EpochCallSource, Unbonding, CONFIG, UNBONDINGS};
use strategy_osmosis_interface::strategy::{
    ExecuteEpochMsg, ExecuteMsg, Phase, QueryMsg, UpdateConfigMsg,
};

use crate::helpers::{register_ica, remove_free_atom_from_host_account, setup};

mod helpers;

// test of the epoch flow
#[test]
fn epoch_deposit_phase_flow() {
    let mut deps = setup();

    // CASE: when the step is 1 as the config is just initialized
    // without any pending deposit
    let epoch_call_source_normal = EpochCallSource::NormalEpoch;
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase_step, 3);

    // TODO: CASE: Step is 1, but, with pending deposit
    // take a step back to 1
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    config.phase_step = 1;
    CONFIG.save(deps.as_mut().storage, &config).unwrap();

    let amount = coins(10000, config.controller_config.deposit_denom.clone());
    // send some funds to the contract
    deps.querier
        .update_balance(mock_env().contract.address, amount);

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase_step, 2);
    // remove funds from contract as it's supposed to be
    deps.querier.update_balance(
        mock_env().contract.address,
        coins(0, config.controller_config.deposit_denom.clone()),
    );

    // CASE: when the step is 2
    let epoch_call_source_transfer = EpochCallSource::TransferCallback;
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_transfer.clone(),
        true,
        None,
    );
    assert!(res.is_ok());

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase_step, 3);

    // CASE: when the step is 3
    // first, register ica_account so that it can be executed properly
    // NOTE: This is totally random address.
    let ica_addr = String::from("osmo1aqvlxpk8dc4m2nkmxkf63a5zez9jkzgm6amkgddhfk0qj9j4rw3q662wuk");
    register_ica(deps.as_mut(), ica_addr);

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase_step, 4);

    // CASE: When the step is 4
    let epoch_call_source_icq = EpochCallSource::IcqCallback;
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_icq.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase_step, 5);

    // CASE: When the step is 5
    // And, when the contract doens't have any deposit toke to be swapped in this step of this phase
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase_step, 7);

    // CASE: with pending deposit
    // take a step back to 5
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    config.phase_step = 5;
    // set some value in to_transfer_to_host in order to test the case when there is pending deposit
    config.host_config.free_base_amount = Uint128::from(1000000u128);
    CONFIG.save(deps.as_mut().storage, &config);

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase_step, 6);
    // remove free_atom_amount since it's supposed to be in real execution
    remove_free_atom_from_host_account(deps.as_mut());

    // CASE: When the step is 6
    // and the callback status is success
    let epoch_call_source_ica = EpochCallSource::IcaCallback;
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_ica.clone(),
        true,
        None,
    );
    assert!(res.is_ok());

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase_step, 7);

    // CASE: When the step is 6 and the callback status is failure
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    config.phase_step = 6;
    CONFIG.save(deps.as_mut().storage, &config);
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_ica.clone(),
        false,
        None,
    );
    assert!(res.is_ok());

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase_step, 5);

    // CASE: When the step is 7
    // take a step forward to 7
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    config.phase_step = 7;
    CONFIG.save(deps.as_mut().storage, &config);

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase_step, 8);

    // CASE: When the step is 8
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_icq.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase_step, 9);

    // CASE: When the step is 9
    // And, when the contract doens't have any deposit toke to be swapped in this step of this phase
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase_step, 11);

    // CASE: When the step is 9 with some amount of free atom in host account
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    config.host_config.free_base_amount = Uint128::from(1000000u128);
    config.phase_step = 9;
    CONFIG.save(deps.as_mut().storage, &config);

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase_step, 10);
    // remove free_atom_amount since it's supposed to be in real execution
    remove_free_atom_from_host_account(deps.as_mut());

    // CASE: when the step is 10
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_ica.clone(),
        true,
        None,
    );
    assert!(res.is_ok());

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase_step, 11);

    // CASE: When the step is 11
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase_step, 12);

    // CASE: When the step is 12
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_icq.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase_step, 13);

    // CASE: when the step is 13
    // And, when the contract doens't have any deposit toke to be swapped in this step of this phase
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase_step, 15);

    // CASE: when the step is 13 and there're unbondinds
    // take a step back to 13
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    config.phase_step = 13;
    CONFIG.save(deps.as_mut().storage, &config);
    // register unbonding
    let sender = deps
        .api
        .addr_validate("ununifi1j9g3qkcxm2xzfc30z462av40vx8kmwakvd00jk")
        .unwrap();
    let unbondings = Unbonding {
        id: 1,
        sender: sender.to_owned(),
        amount: Uint128::from(100 as u32),
        pending_start: false,
        start_time: 0,
        marked: false,
    };
    UNBONDINGS.save(deps.as_mut().storage, 1, &unbondings);

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase_step, 14);

    // CASE: when the step is 14 in ica_call_back
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_ica.clone(),
        true,
        None,
    );
    assert!(res.is_ok());

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase_step, 15);

    // CASE: When the step is 15
    // And when  free lp amount is 0
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase, Phase::Deposit);
    assert_eq!(config.phase_step, 1);

    // CASE: When the step is 15
    // And when  free lp amount is not 0 and matured unbondings are not empty
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    config.host_config.free_lp_amount = Uint128::from(100 as u32);
    // take a step back to 15
    config.phase_step = 15;
    // change unbonding_time to useful configure for this test
    config.unbond_period = 1;
    CONFIG.save(deps.as_mut().storage, &config);
    let sender = deps
        .api
        .addr_validate("ununifi1j9g3qkcxm2xzfc30z462av40vx8kmwakvd00jk")
        .unwrap();
    let unbondings = Unbonding {
        id: 1,
        sender: sender.to_owned(),
        amount: Uint128::from(100 as u32),
        pending_start: false,
        start_time: 0,
        marked: false,
    };
    UNBONDINGS.save(deps.as_mut().storage, 1, &unbondings);

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase, Phase::Withdraw);
    assert_eq!(config.phase_step, 1);
}

// test of the step flow in Withdraw phase
#[test]
fn epoch_withdraw_phase_flow() {
    let mut deps = setup();
    let epoch_call_source_normal = EpochCallSource::NormalEpoch;
    let epoch_call_source_icq = EpochCallSource::IcqCallback;
    let epoch_call_source_ica = EpochCallSource::IcaCallback;
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    config.phase = Phase::Withdraw;
    CONFIG.save(deps.as_mut().storage, &config).unwrap();

    // CASE: when the step is 1 as the config is just initialized
    // without any pending deposit
    setup_test_case_for_execute_epoch(deps.as_mut(), 1, Uint128::zero());
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    assert_config_phase_step(deps.as_mut(), 2);

    // CASE: when the step is 2
    setup_test_case_for_execute_epoch(deps.as_mut(), 2, Uint128::zero());
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_ica.clone(),
        true,
        None,
    );
    assert!(res.is_ok());

    assert_config_phase_step(deps.as_mut(), 3);

    // CASE: when the step is 3
    // first, register ica_account so that it can be executed properly
    // NOTE: This is totally random address.
    let ica_addr = String::from("osmo1aqvlxpk8dc4m2nkmxkf63a5zez9jkzgm6amkgddhfk0qj9j4rw3q662wuk");
    register_ica(deps.as_mut(), ica_addr);

    setup_test_case_for_execute_epoch(deps.as_mut(), 3, Uint128::zero());
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());

    assert_config_phase_step(deps.as_mut(), 4);

    // CASE: When the step is 4
    setup_test_case_for_execute_epoch(deps.as_mut(), 4, Uint128::zero());
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_icq.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    assert_config_phase_step(deps.as_mut(), 5);

    // CASE: When the step is 5
    // And, when
    setup_test_case_for_execute_epoch(deps.as_mut(), 5, Uint128::zero());
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    assert_config_phase_step(deps.as_mut(), 6);

    // CASE: When the step is 6
    // And, the ica tx is success
    setup_test_case_for_execute_epoch(deps.as_mut(), 6, Uint128::zero());
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_ica.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    assert_config_phase_step(deps.as_mut(), 7);

    // CASE: When the step is 6
    // And, the ica tx is failure
    setup_test_case_for_execute_epoch(deps.as_mut(), 6, Uint128::zero());
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_ica.clone(),
        false,
        None,
    );
    assert!(res.is_ok());
    assert_config_phase_step(deps.as_mut(), 5);

    // CASE: When the step is 7
    setup_test_case_for_execute_epoch(deps.as_mut(), 7, Uint128::zero());
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    assert_config_phase_step(deps.as_mut(), 8);

    // CASE: When the step is 8
    setup_test_case_for_execute_epoch(deps.as_mut(), 8, Uint128::zero());

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_icq.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    assert_config_phase_step(deps.as_mut(), 9);

    // CASE: When the step is 9
    setup_test_case_for_execute_epoch(deps.as_mut(), 9, Uint128::zero());

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    assert_config_phase_step(deps.as_mut(), 10);

    // CASE: When the step is 10
    // And, ica tx is success
    setup_test_case_for_execute_epoch(deps.as_mut(), 10, Uint128::zero());

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_ica.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    assert_config_phase_step(deps.as_mut(), 11);

    // CASE: When the step is 10
    // And, ica tx is failure
    setup_test_case_for_execute_epoch(deps.as_mut(), 10, Uint128::zero());

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_ica.clone(),
        false,
        None,
    );
    assert!(res.is_ok());
    assert_config_phase_step(deps.as_mut(), 9);

    // CASE: When the step is 11
    setup_test_case_for_execute_epoch(deps.as_mut(), 11, Uint128::zero());

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    assert_config_phase_step(deps.as_mut(), 12);

    // CASE: When the step is 12
    setup_test_case_for_execute_epoch(deps.as_mut(), 12, Uint128::zero());

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_icq.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    assert_config_phase_step(deps.as_mut(), 13);

    // CASE: When the step is 13
    setup_test_case_for_execute_epoch(deps.as_mut(), 13, Uint128::zero());

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    assert_config_phase_step(deps.as_mut(), 1);
    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase, Phase::Deposit);
}

fn setup_test_case_for_execute_epoch(deps: DepsMut, phase_step: u64, free_atom_amount: Uint128) {
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    config.phase_step = phase_step;
    config.host_config.free_base_amount = free_atom_amount;

    CONFIG.save(deps.storage, &config).unwrap();
}

fn assert_config_phase_step(deps: DepsMut, expected_phase_step: u64) {
    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.phase_step, expected_phase_step);
}