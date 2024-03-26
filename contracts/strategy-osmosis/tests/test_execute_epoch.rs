use cosmwasm_std::testing::mock_env;
use cosmwasm_std::{coins, Api, DepsMut, Uint128};
use helpers::th_query;
use strategy_osmosis::execute::epoch::epoch::execute_epoch;
use strategy_osmosis::msgs::{Phase, PhaseStep, QueryMsg};
use strategy_osmosis::state::{
    EpochCallSource, Params, State, Unbonding, PARAMS, STATE, UNBONDINGS,
};

use crate::helpers::{register_ica, remove_free_atom_from_host_account, setup};

mod helpers;

// test of the epoch flow
#[test]
fn epoch_deposit_phase_flow() {
    let mut deps = setup();

    // CASE: when the step is PhaseStep::IbcTransferToHost as the params is just initialized
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

    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(
        params.phase_step,
        PhaseStep::RequestIcqAfterIbcTransferToHost
    );

    // TODO: CASE: Step is PhaseStep::IbcTransferToHost, but, with pending deposit
    // take a step back to PhaseStep::IbcTransferToHost
    let mut params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    params.phase_step = PhaseStep::IbcTransferToHost;
    PARAMS.save(deps.as_mut().storage, &params).unwrap();
    let mut state: State = th_query(deps.as_ref(), QueryMsg::State {});
    state.controller_stacked_amount_to_deposit = Uint128::from(10000u128);
    _ = STATE.save(deps.as_mut().storage, &state);

    let amount = coins(10000, params.controller_deposit_denom.clone());
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

    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(params.phase_step, PhaseStep::IbcTransferToHostCallback);

    // remove funds from contract as it's supposed to be
    deps.querier.update_balance(
        mock_env().contract.address,
        coins(0, params.controller_deposit_denom.clone()),
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

    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(
        params.phase_step,
        PhaseStep::RequestIcqAfterIbcTransferToHost
    );

    // CASE: when the step is PhaseStep::RequestIcqAfterIbcTransferToHost
    // first, register ica_account so that it can be executed properly
    // NOTE: This is totally random address.
    let ica_addr = String::from("osmo1aqvlxpk8dc4m2nkmxkf63a5zez9jkzgm6amkgddhfk0qj9j4rw3q662wuk");
    _ = register_ica(deps.as_mut(), ica_addr);

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());

    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(
        params.phase_step,
        PhaseStep::ResponseIcqAfterIbcTransferToHost
    );

    // CASE: When the step is PhaseStep::ResponseIcqAfterIbcTransferToHost
    let epoch_call_source_icq = EpochCallSource::IcqCallback;
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_icq.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(params.phase_step, PhaseStep::AddLiquidity);

    // CASE: When the step is PhaseStep::AddLiquidity
    // And, when the contract doens't have any deposit toke to be swapped in this step of this phase
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(
        params.phase_step,
        PhaseStep::BeginUnbondingForPendingRequests
    );

    // CASE: with pending deposit
    // take a step back to PhaseStep::AddLiquidity
    let mut params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    params.phase_step = PhaseStep::AddLiquidity;
    _ = PARAMS.save(deps.as_mut().storage, &params);
    let mut state: State = th_query(deps.as_ref(), QueryMsg::State {});
    // set some value in to_transfer_to_host in order to test the case when there is pending deposit
    state.free_base_amount = Uint128::from(1000000u128);
    _ = STATE.save(deps.as_mut().storage, &state);

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());

    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(params.phase_step, PhaseStep::AddLiquidityCallback);
    // remove free_atom_amount since it's supposed to be in real execution
    remove_free_atom_from_host_account(deps.as_mut());

    // CASE: When the step is PhaseStep::AddLiquidityCallback
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

    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(params.phase_step, PhaseStep::BondLiquidity);

    // CASE: When the step is 6 and the callback status is failure
    let mut params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    params.phase_step = PhaseStep::AddLiquidityCallback;
    _ = PARAMS.save(deps.as_mut().storage, &params);
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_ica.clone(),
        false,
        None,
    );
    assert!(res.is_ok());

    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(params.phase_step, PhaseStep::AddLiquidity);

    // CASE: When the step is PhaseStep::BondLiquidity
    // take a step forward to PhaseStep::BondLiquidity
    let mut params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    params.phase_step = PhaseStep::BondLiquidity;
    _ = PARAMS.save(deps.as_mut().storage, &params);
    let mut state: State = th_query(deps.as_ref(), QueryMsg::State {});
    state.pending_bond_lp_amount = Uint128::from(100000u128);
    _ = STATE.save(deps.as_mut().storage, &state);

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());

    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(params.phase_step, PhaseStep::BondLiquidityCallback);

    // CASE: When the step is PhaseStep::BondLiquidityCallback
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_ica.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(params.phase_step, PhaseStep::RequestIcqAfterBondLiquidity);

    // remove free_atom_amount since it's supposed to be in real execution
    remove_free_atom_from_host_account(deps.as_mut());

    // CASE: when the step is PhaseStep::ResponseIcqAfterBondLiquidity
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_ica.clone(),
        true,
        None,
    );
    assert!(res.is_ok());

    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(params.phase_step, PhaseStep::ResponseIcqAfterBondLiquidity);

    // CASE: When the step is PhaseStep::ResponseIcqAfterBondLiquidity
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(params.phase_step, PhaseStep::ResponseIcqAfterBondLiquidity);

    // CASE: When the step is PhaseStep::BeginUnbondingForPendingRequestsCallback
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_icq.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(
        params.phase_step,
        PhaseStep::BeginUnbondingForPendingRequests
    );

    // CASE: when the step is PhaseStep::BeginUnbondingForPendingRequests
    // And, when the contract doens't have any deposit toke to be swapped in this step of this phase
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(params.phase_step, PhaseStep::CheckMaturedUnbondings);

    // CASE: when the step is PhaseStep::BeginUnbondingForPendingRequests and there're unbondinds
    // take a step back to PhaseStep::BeginUnbondingForPendingRequests
    let mut params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    params.phase_step = PhaseStep::BeginUnbondingForPendingRequests;
    _ = PARAMS.save(deps.as_mut().storage, &params);
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
    _ = UNBONDINGS.save(deps.as_mut().storage, 1, &unbondings);

    let mut state: State = th_query(deps.as_ref(), QueryMsg::State {});
    state.unbond_request_lp_amount = Uint128::from(100u128);
    _ = STATE.save(deps.as_mut().storage, &state);

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());

    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(
        params.phase_step,
        PhaseStep::BeginUnbondingForPendingRequestsCallback
    );

    // CASE: when the step is 12 in ica_call_back
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_ica.clone(),
        true,
        None,
    );
    assert!(res.is_ok());

    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(params.phase_step, PhaseStep::CheckMaturedUnbondings);

    // CASE: When the step is 13
    // And when  free lp amount is 0
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(params.phase, Phase::Deposit);
    assert_eq!(params.phase_step, PhaseStep::IbcTransferToHost);

    // CASE: When the step is PhaseStep::CheckMaturedUnbondings
    // And when  free lp amount is not 0 and matured unbondings are not empty
    let mut params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    // take a step back to PhaseStep::CheckMaturedUnbondings
    params.phase_step = PhaseStep::CheckMaturedUnbondings;

    // change unbonding_time to useful paramsure for this test
    params.unbond_period = 1;
    _ = PARAMS.save(deps.as_mut().storage, &params);

    let mut state: State = th_query(deps.as_ref(), QueryMsg::State {});
    state.free_lp_amount = Uint128::from(100 as u32);
    _ = STATE.save(deps.as_mut().storage, &state);

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
    _ = UNBONDINGS.save(deps.as_mut().storage, 1, &unbondings);

    _ = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(params.phase, Phase::Withdraw);
    assert_eq!(params.phase_step, PhaseStep::RemoveLiquidity);
}

// test of the step flow in Withdraw phase
#[test]
fn epoch_withdraw_phase_flow() {
    let mut deps = setup();
    let epoch_call_source_normal = EpochCallSource::NormalEpoch;
    let epoch_call_source_icq = EpochCallSource::IcqCallback;
    let epoch_call_source_ica = EpochCallSource::IcaCallback;
    let mut params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    params.phase = Phase::Withdraw;
    PARAMS.save(deps.as_mut().storage, &params).unwrap();

    // CASE: when the step is PhaseStep::RemoveLiquidity as the params is just initialized
    // without any pending deposit
    setup_test_case_for_execute_epoch(deps.as_mut(), PhaseStep::RemoveLiquidity, Uint128::zero());

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    assert_params_phase_step(deps.as_mut(), PhaseStep::RemoveLiquidityCallback);

    // CASE: when the step is 2
    setup_test_case_for_execute_epoch(
        deps.as_mut(),
        PhaseStep::RemoveLiquidityCallback,
        Uint128::zero(),
    );
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_ica.clone(),
        true,
        None,
    );
    assert!(res.is_ok());

    assert_params_phase_step(deps.as_mut(), PhaseStep::RequestIcqAfterRemoveLiquidity);

    // CASE: when the step is 3
    // first, register ica_account so that it can be executed properly
    // NOTE: This is totally random address.
    let ica_addr = String::from("osmo1aqvlxpk8dc4m2nkmxkf63a5zez9jkzgm6amkgddhfk0qj9j4rw3q662wuk");
    _ = register_ica(deps.as_mut(), ica_addr);

    setup_test_case_for_execute_epoch(
        deps.as_mut(),
        PhaseStep::RequestIcqAfterRemoveLiquidity,
        Uint128::zero(),
    );
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());

    assert_params_phase_step(deps.as_mut(), PhaseStep::ResponseIcqAfterRemoveLiquidity);

    // CASE: When the step is PhaseStep::ResponseIcqAfterRemoveLiquidity
    setup_test_case_for_execute_epoch(
        deps.as_mut(),
        PhaseStep::ResponseIcqAfterRemoveLiquidity,
        Uint128::zero(),
    );
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_icq.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    assert_params_phase_step(deps.as_mut(), PhaseStep::SwapTwoTokensToDepositToken);

    // CASE: When the step is PhaseStep::SwapTwoTokensToDepositToken
    // And, when
    setup_test_case_for_execute_epoch(
        deps.as_mut(),
        PhaseStep::SwapTwoTokensToDepositToken,
        Uint128::zero(),
    );
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    assert_params_phase_step(
        deps.as_mut(),
        PhaseStep::SwapTwoTokensToDepositTokenCallback,
    );

    // CASE: When the step is 6
    // And, the ica tx is success
    setup_test_case_for_execute_epoch(
        deps.as_mut(),
        PhaseStep::SwapTwoTokensToDepositTokenCallback,
        Uint128::zero(),
    );
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_ica.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    assert_params_phase_step(
        deps.as_mut(),
        PhaseStep::RequestIcqAfterSwapTwoTokensToDepositToken,
    );

    // CASE: When the step is PhaseStep::SwapTwoTokensToDepositTokenCallback
    // And, the ica tx is failure
    setup_test_case_for_execute_epoch(
        deps.as_mut(),
        PhaseStep::SwapTwoTokensToDepositTokenCallback,
        Uint128::zero(),
    );
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_ica.clone(),
        false,
        None,
    );
    assert!(res.is_ok());
    assert_params_phase_step(deps.as_mut(), PhaseStep::SwapTwoTokensToDepositToken);

    // CASE: When the step is PhaseStep::RequestIcqAfterSwapTwoTokensToDepositToken
    setup_test_case_for_execute_epoch(
        deps.as_mut(),
        PhaseStep::RequestIcqAfterSwapTwoTokensToDepositToken,
        Uint128::zero(),
    );
    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    assert_params_phase_step(
        deps.as_mut(),
        PhaseStep::ResponseIcqAfterSwapTwoTokensToDepositToken,
    );

    // CASE: When the step is PhaseStep::ResponseIcqAfterSwapTwoTokensToDepositToken
    setup_test_case_for_execute_epoch(
        deps.as_mut(),
        PhaseStep::ResponseIcqAfterSwapTwoTokensToDepositToken,
        Uint128::zero(),
    );

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_icq.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    assert_params_phase_step(deps.as_mut(), PhaseStep::IbcTransferToController);

    // CASE: When the step is PhaseStep::IbcTransferToController
    setup_test_case_for_execute_epoch(
        deps.as_mut(),
        PhaseStep::IbcTransferToController,
        Uint128::zero(),
    );

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    assert_params_phase_step(deps.as_mut(), PhaseStep::IbcTransferToControllerCallback);

    // CASE: When the step is PhaseStep::IbcTransferToControllerCallback
    // And, ica tx is success
    setup_test_case_for_execute_epoch(
        deps.as_mut(),
        PhaseStep::IbcTransferToControllerCallback,
        Uint128::zero(),
    );

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_ica.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    assert_params_phase_step(
        deps.as_mut(),
        PhaseStep::RequestIcqAfterIbcTransferToController,
    );

    // CASE: When the step is PhaseStep::IbcTransferToControllerCallback
    // And, ica tx is failure
    setup_test_case_for_execute_epoch(
        deps.as_mut(),
        PhaseStep::IbcTransferToControllerCallback,
        Uint128::zero(),
    );

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_ica.clone(),
        false,
        None,
    );
    assert!(res.is_ok());
    assert_params_phase_step(deps.as_mut(), PhaseStep::IbcTransferToController);

    // CASE: When the step is 11
    setup_test_case_for_execute_epoch(
        deps.as_mut(),
        PhaseStep::RequestIcqAfterIbcTransferToController,
        Uint128::zero(),
    );

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    assert_params_phase_step(
        deps.as_mut(),
        PhaseStep::ResponseIcqAfterIbcTransferToController,
    );

    // CASE: When the step is PhaseStep::ResponseIcqAfterIbcTransferToController
    setup_test_case_for_execute_epoch(
        deps.as_mut(),
        PhaseStep::ResponseIcqAfterIbcTransferToController,
        Uint128::zero(),
    );

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_icq.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    assert_params_phase_step(deps.as_mut(), PhaseStep::DistributeToUnbondedUsers);

    // CASE: When the step is PhaseStep::DistributeToUnbondedUsers
    setup_test_case_for_execute_epoch(
        deps.as_mut(),
        PhaseStep::DistributeToUnbondedUsers,
        Uint128::zero(),
    );

    let res = execute_epoch(
        deps.as_mut(),
        mock_env(),
        epoch_call_source_normal.clone(),
        true,
        None,
    );
    assert!(res.is_ok());
    assert_params_phase_step(deps.as_mut(), PhaseStep::IbcTransferToHost);
    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(params.phase, Phase::Deposit);
}

fn setup_test_case_for_execute_epoch(
    deps: DepsMut,
    phase_step: PhaseStep,
    free_atom_amount: Uint128,
) {
    let mut params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    params.phase_step = phase_step;
    PARAMS.save(deps.storage, &params).unwrap();

    let mut state: State = th_query(deps.as_ref(), QueryMsg::State {});
    state.free_base_amount = free_atom_amount;
    _ = STATE.save(deps.storage, &state);
}

fn assert_params_phase_step(deps: DepsMut, expected_phase_step: PhaseStep) {
    let params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    assert_eq!(params.phase_step, expected_phase_step);
}
