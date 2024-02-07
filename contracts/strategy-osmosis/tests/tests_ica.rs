use crate::helpers::{setup, th_query};
use cosmwasm_std::testing::mock_env;
use cosmwasm_std::{from_binary, Addr, CosmosMsg, IbcMsg, Uint128};
use ica_tx::helpers::{send_ica_tx, InterchainAccountPacketData};
use osmosis_std::types::osmosis::gamm::v1beta1::MsgJoinPool;
use prost::Message;
use prost_types::Any;
use strategy_osmosis::execute::epoch::helpers::determine_ica_amounts;
use strategy_osmosis::execute::epoch::liquidity::execute_ica_join_swap_extern_amount_in;
use strategy_osmosis::execute::epoch::token_transfer::execute_ibc_transfer_to_controller;
use strategy_osmosis::helpers::join_pool_to_any;
use strategy_osmosis::msgs::{Phase, PhaseStep, QueryMsg};
use strategy_osmosis::state::{DepositToken, Params, State, PARAMS, STAKE_RATE_MULTIPLIER, STATE};
mod helpers;
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmosisCoin;

#[test]
fn determine_ica_amounts_for_deposit() {
    // Phase is Deposit
    let params = Params {
        authority: Addr::unchecked("authority"),
        unbond_period: 0,
        phase: Phase::Deposit,
        phase_step: PhaseStep::IbcTransferToHost,
        chain_id: "test-1".to_string(),
        pool_id: 1,
        deposit_token: DepositToken::Base,
        controller_deposit_denom: "stake".to_string(), // `ibc/xxxxuatom`
        quote_denom: "uosmo".to_string(),
        base_denom: "stake".to_string(),
        lp_denom: "gamm/pool/1".to_string(),
        transfer_timeout: 300,
        transfer_channel_id: "channel-1".to_string(),
        controller_transfer_channel_id: "channel-1".to_string(),
        ica_channel_id: "".to_string(),
        ica_connection_id: "".to_string(),
        ica_account: "".to_string(),
        superfluid_validator: "".to_string(),
        automate_superfluid: true,
        extern_tokens: vec![],
    };

    let state = State {
        last_unbonding_id: 1u64,
        redemption_rate: STAKE_RATE_MULTIPLIER,
        total_shares: Uint128::from(0u128),
        total_deposit: Uint128::from(0u128),
        total_withdrawn: Uint128::from(0u128),
        pending_icq: 0u64,
        lp_redemption_rate: Uint128::from(2u128),
        lock_id: 0u64,
        bonded_lp_amount: Uint128::from(0u128),
        unbond_request_lp_amount: Uint128::from(0u128),
        unbonding_lp_amount: Uint128::from(0u128),
        free_lp_amount: Uint128::from(0u128),
        pending_bond_lp_amount: Uint128::from(0u128),
        pending_lp_removal_amount: Uint128::from(0u128),
        free_quote_amount: Uint128::from(5000u128),
        free_base_amount: Uint128::from(10000u128),
        controller_free_amount: Uint128::from(10000u128),
        controller_pending_transfer_amount: Uint128::from(0u128),
        controller_stacked_amount_to_deposit: Uint128::from(10000u128),
        extern_token_amounts: vec![],
    };

    let ica_amounts = determine_ica_amounts(params, state);

    // NOTE: below nums are just filled in by refering to the code.
    // Of course, this doesn't assure the code itself is designed as intended
    assert_eq!(ica_amounts.to_swap_amount, Uint128::from(0u128));
    assert_eq!(ica_amounts.to_transfer_to_host, Uint128::from(10000u128));
}

#[test]
fn determine_ica_amounts_for_withdraw() {
    let params = Params {
        phase: Phase::Withdraw,
        // unused fields
        chain_id: "test-1".to_string(),
        pool_id: 1,
        transfer_channel_id: "channel-1".to_string(),
        lp_denom: "gamm/pool/1".to_string(),
        quote_denom: "uosmo".to_string(),
        base_denom: "stake".to_string(),
        controller_transfer_channel_id: "channel-1".to_string(),
        controller_deposit_denom: "stake".to_string(), // `ibc/xxxxuatom`
        authority: Addr::unchecked("authority"),
        deposit_token: DepositToken::Base,
        unbond_period: 0,
        transfer_timeout: 300,
        ica_connection_id: "".to_string(),
        ica_channel_id: "".to_string(),
        ica_account: "".to_string(),
        phase_step: PhaseStep::RemoveLiquidity,
        superfluid_validator: "".to_string(),
        automate_superfluid: true,
        extern_tokens: vec![],
    };

    let state = State {
        lock_id: 0u64,
        bonded_lp_amount: Uint128::from(0u128),
        unbond_request_lp_amount: Uint128::from(0u128),
        unbonding_lp_amount: Uint128::from(0u128),
        pending_bond_lp_amount: Uint128::from(0u128),
        pending_lp_removal_amount: Uint128::from(0u128),
        controller_free_amount: Uint128::from(10000u128),
        controller_pending_transfer_amount: Uint128::from(0u128),
        controller_stacked_amount_to_deposit: Uint128::from(1000u128),
        lp_redemption_rate: Uint128::from(2u128),
        free_base_amount: Uint128::from(10000u128),
        free_lp_amount: Uint128::from(10u128),
        free_quote_amount: Uint128::from(5000u128),
        total_deposit: Uint128::from(0u128),
        last_unbonding_id: 1u64,
        redemption_rate: STAKE_RATE_MULTIPLIER,
        total_shares: Uint128::from(0u128),
        total_withdrawn: Uint128::from(0u128),
        pending_icq: 0u64,
        extern_token_amounts: vec![],
    };
    let ica_amounts = determine_ica_amounts(params, state);

    // NOTE: below nums are just filled in by refering to the code.
    // Of course, this doesn't assure the code itself is designed as intended
    assert_eq!(ica_amounts.to_swap_amount, Uint128::from(5000u128));
    assert_eq!(ica_amounts.to_remove_lp, Uint128::from(10u128));
    assert_eq!(
        ica_amounts.to_transfer_to_controller,
        Uint128::from(10000u128)
    );
    assert_eq!(ica_amounts.to_return_amount, Uint128::from(9000u128));
}

#[test]
fn test_execute_transfer_to_controller() {
    let mut deps = setup();

    // When is to_transfer_to_controller is zero.
    let res = execute_ibc_transfer_to_controller(deps.as_mut().storage, mock_env());
    assert!(res.is_ok());
    assert_eq!(res.as_ref().unwrap().messages.len(), 0);

    // When is to_transfer_to_controller is not zero.
    let mut params: Params = th_query(deps.as_ref(), QueryMsg::Params {});
    params.phase = Phase::Withdraw;
    PARAMS.save(deps.as_mut().storage, &params).unwrap();

    let mut state: State = th_query(deps.as_ref(), QueryMsg::State {});
    state.free_base_amount = Uint128::from(10000u128);
    _ = STATE.save(deps.as_mut().storage, &state);

    let res = execute_ibc_transfer_to_controller(deps.as_mut().storage, mock_env());
    assert!(res.is_ok());
    assert_eq!(res.as_ref().unwrap().messages.len(), 1);
}

// test of execute_ica_add_liquidity
#[test]
fn test_execute_ica_add_liquidity() {
    let mut deps = setup();

    // When is to_transfer_to_controller is zero.
    let res = execute_ica_join_swap_extern_amount_in(deps.as_mut().storage, mock_env());
    assert!(res.is_ok());
    assert_eq!(res.as_ref().unwrap().messages.len(), 0);

    // When is to_transfer_to_controller is not zero.
    let mut state: State = th_query(deps.as_ref(), QueryMsg::State {});
    state.free_base_amount = Uint128::from(100000u128);
    _ = STATE.save(deps.as_mut().storage, &state);

    let res = execute_ica_join_swap_extern_amount_in(deps.as_mut().storage, mock_env());
    assert!(res.is_ok());
    assert_eq!(res.as_ref().unwrap().messages.len(), 1);
}

#[test]
fn test_send_ica_tx() {
    let sender = Addr::unchecked("sender");

    let tokens_in: Vec<OsmosisCoin> = vec![
        OsmosisCoin {
            denom: "uosmo".to_string(),
            amount: Uint128::from(100000u128).to_string(),
        },
        OsmosisCoin {
            denom: "uatom".to_string(),
            amount: Uint128::from(100000u128).to_string(),
        },
    ];

    let msg1 = MsgJoinPool {
        sender: sender.to_string(),
        share_out_amount: Uint128::from(100000u128).to_string(),
        pool_id: 1,
        token_in_maxs: tokens_in,
    };

    _ = setup();
    let res = send_ica_tx(
        mock_env(),
        "channel-1".to_string(),
        300,
        "test".to_string(),
        vec![join_pool_to_any(msg1.clone()).unwrap()],
    );

    assert!(res.is_ok());
    assert_eq!(res.as_ref().unwrap().messages.len(), 1);

    match &res.as_ref().unwrap().messages[0].msg {
        CosmosMsg::Ibc(IbcMsg::SendPacket { data, .. }) => {
            let packet_data: InterchainAccountPacketData = from_binary(&data.clone()).unwrap();
            let cosmos_tx = Any::decode(packet_data.data.as_slice()).unwrap();
            assert!(cosmos_tx.value.is_empty());
            // NOTE: below type_url is just filled in by refering to the result of the function.
            assert_eq!(cosmos_tx.type_url, "\n!/osmosis.gamm.v1beta1.MsgJoinPool\u{12}4\n\u{6}sender\u{10}\u{1}\u{1a}\u{6}100000\"\u{f}\n\u{5}uosmo\u{12}\u{6}100000\"\u{f}\n\u{5}uatom\u{12}\u{6}100000");
        }
        _ => panic!("Unexpected message type"),
    }
}
