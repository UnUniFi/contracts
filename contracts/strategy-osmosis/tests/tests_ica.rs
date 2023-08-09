use crate::helpers::{setup, th_query};
use cosmwasm_std::testing::mock_env;
use cosmwasm_std::{from_binary, Addr, CosmosMsg, IbcMsg, Uint128};
use ica_tx::helpers::{send_ica_tx, InterchainAccountPacketData};
use osmosis_std::types::osmosis::gamm::v1beta1::MsgJoinPool;
use prost::Message;
use prost_types::Any;
use strategy_osmosis::helpers::join_pool_to_any;
use strategy_osmosis::ica::{
    determine_ica_amounts, execute_ibc_transfer_to_controller,
    execute_ica_join_swap_extern_amount_in,
};
use strategy_osmosis::msgs::{Phase, PhaseStep, QueryMsg};
use strategy_osmosis::state::{
    Config, ControllerConfig, HostConfig, CONFIG, STAKE_RATE_MULTIPLIER,
};
mod helpers;
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmosisCoin;

#[test]
fn determine_ica_amounts_for_deposit() {
    // Phase is Deposit
    let config = Config {
        phase: Phase::Deposit,
        host_config: HostConfig {
            lp_redemption_rate: Uint128::from(2u128),
            free_base_amount: Uint128::from(10000u128),
            free_lp_amount: Uint128::from(0u128),
            free_quote_amount: Uint128::from(5000u128),

            // unused fields
            chain_id: "test-1".to_string(),
            pool_id: 1,
            transfer_channel_id: "channel-1".to_string(),
            lock_id: 0u64,
            lp_denom: "gamm/pool/1".to_string(),
            bonded_lp_amount: Uint128::from(0u128),
            unbonding_lp_amount: Uint128::from(0u128),
            pending_bond_lp_amount: Uint128::from(0u128),
            pending_lp_removal_amount: Uint128::from(0u128),
            quote_denom: "uosmo".to_string(),
            pending_swap_to_base_amount: Uint128::from(0u128),
            base_denom: "stake".to_string(),
            pending_swap_to_quote_amount: Uint128::from(0u128),
            pending_add_liquidity_amount: Uint128::from(0u128),
            pending_transfer_amount: Uint128::from(0u128), // pending transfer to controller - TODO: how to get hook for transfer finalization?
        },
        controller_config: ControllerConfig {
            free_amount: Uint128::from(10000u128),
            pending_transfer_amount: Uint128::from(0u128),
            stacked_amount_to_deposit: Uint128::from(0u128),

            // unused fields
            transfer_channel_id: "channel-1".to_string(),
            deposit_denom: "stake".to_string(), // `ibc/xxxxuatom`
        }, // unused fields
        owner: Addr::unchecked("owner"),
        unbond_period: 0,
        total_deposit: Uint128::from(0u128),
        last_unbonding_id: 1u64,
        redemption_rate: STAKE_RATE_MULTIPLIER,
        total_shares: Uint128::from(0u128),
        total_withdrawn: Uint128::from(0u128),
        transfer_timeout: 300,
        ica_connection_id: "".to_string(),
        ica_channel_id: "".to_string(),
        ica_account: "".to_string(),
        phase_step: PhaseStep::IbcTransferToHost,
        pending_icq: 0u64,
    };

    let ica_amounts = determine_ica_amounts(config);

    // NOTE: below nums are just filled in by refering to the code.
    // Of course, this doesn't assure the code itself is designed as intended
    assert_eq!(ica_amounts.to_swap_base, Uint128::from(5000u128));
    assert_eq!(ica_amounts.to_swap_quote, Uint128::from(2500u128));
    assert_eq!(ica_amounts.to_transfer_to_host, Uint128::from(10000u128));
}

#[test]
fn determine_ica_amounts_for_withdraw() {
    let config = Config {
        phase: Phase::Withdraw,
        host_config: HostConfig {
            lp_redemption_rate: Uint128::from(2u128),
            free_base_amount: Uint128::from(10000u128),
            free_lp_amount: Uint128::from(10u128),
            free_quote_amount: Uint128::from(5000u128),

            // unused fields
            chain_id: "test-1".to_string(),
            pool_id: 1,
            transfer_channel_id: "channel-1".to_string(),
            lock_id: 0u64,
            lp_denom: "gamm/pool/1".to_string(),
            bonded_lp_amount: Uint128::from(0u128),
            unbonding_lp_amount: Uint128::from(0u128),
            pending_bond_lp_amount: Uint128::from(0u128),
            pending_lp_removal_amount: Uint128::from(0u128),
            quote_denom: "uosmo".to_string(),
            pending_swap_to_base_amount: Uint128::from(0u128),
            base_denom: "stake".to_string(),
            pending_swap_to_quote_amount: Uint128::from(0u128),
            pending_add_liquidity_amount: Uint128::from(0u128),
            pending_transfer_amount: Uint128::from(0u128), // pending transfer to controller - TODO: how to get hook for transfer finalization?
        },
        controller_config: ControllerConfig {
            free_amount: Uint128::from(10000u128),
            pending_transfer_amount: Uint128::from(0u128),
            stacked_amount_to_deposit: Uint128::from(1000u128),

            // unused fields
            transfer_channel_id: "channel-1".to_string(),
            deposit_denom: "stake".to_string(), // `ibc/xxxxuatom`
        }, // unused fields
        owner: Addr::unchecked("owner"),
        unbond_period: 0,
        total_deposit: Uint128::from(0u128),
        last_unbonding_id: 1u64,
        redemption_rate: STAKE_RATE_MULTIPLIER,
        total_shares: Uint128::from(0u128),
        total_withdrawn: Uint128::from(0u128),
        transfer_timeout: 300,
        ica_connection_id: "".to_string(),
        ica_channel_id: "".to_string(),
        ica_account: "".to_string(),
        phase_step: PhaseStep::RemoveLiquidity,
        pending_icq: 0u64,
    };

    let ica_amounts = determine_ica_amounts(config);

    // NOTE: below nums are just filled in by refering to the code.
    // Of course, this doesn't assure the code itself is designed as intended
    assert_eq!(ica_amounts.to_swap_base, Uint128::zero());
    assert_eq!(ica_amounts.to_swap_quote, Uint128::from(10000u128));
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
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    config.phase = Phase::Withdraw;
    config.host_config.free_base_amount = Uint128::from(10000u128);
    CONFIG.save(deps.as_mut().storage, &config).unwrap();

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
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    config.host_config.free_base_amount = Uint128::from(100000u128);
    CONFIG.save(deps.as_mut().storage, &config).unwrap();

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
