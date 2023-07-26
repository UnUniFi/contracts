use cosmwasm_std::testing::{mock_env, mock_dependencies};
use cosmwasm_std::{Uint128, Addr, to_binary, IbcMsg, CosmosMsg, from_binary, SubMsg, Binary};
use osmosis_std::types::osmosis::gamm::v1beta1::MsgJoinPool;
use prost::Message;
use prost_types::Any;
use proto::ibc::applications::interchain_accounts::v1::CosmosTx;
// use cosmwasm_std::Overflow;
// use osmosis_std::types::osmosis::epochs::v1beta1::EpochInfo;
use strategy_osmosis::strategy::{Phase, QueryMsg};
use strategy_osmosis_atom_osmo::helpers::send_ica_tx;
use strategy_osmosis_atom_osmo::ica::{determine_ica_amounts, execute_ibc_transfer_to_controller, execute_ica_add_and_bond_liquidity};
use strategy_osmosis_atom_osmo::state::{Config, STAKE_RATE_MULTIPLIER, HostConfig, ControllerConfig, CONFIG, InterchainAccountPacketData};
use crate::helpers::{setup, th_query, join_pool_to_any};
mod helpers;
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmosisCoin;

#[test]
fn determine_ica_amounts_for_deposit() {
    // Phase is Deposit
    let config = Config {
        phase: Phase::Deposit,
        host_config: HostConfig {
            lp_redemption_rate: Uint128::from(2u128),
            free_atom_amount: Uint128::from(10000u128),
            free_lp_amount: Uint128::from(0u128),
            free_osmo_amount: Uint128::from(5000u128),

            // unused fields
            chain_id: "test-1".to_string(),pool_id: 1,transfer_channel_id: "channel-1".to_string(),lock_id: 0u64,lp_denom: "gamm/pool/1".to_string(),bonded_lp_amount: Uint128::from(0u128),pending_bond_lp_amount: Uint128::from(0u128),pending_lp_removal_amount: Uint128::from(0u128), osmo_denom: "uosmo".to_string(),pending_swap_to_atom_amount: Uint128::from(0u128), atom_denom: "stake".to_string(),pending_swap_to_osmo_amount: Uint128::from(0u128), pending_add_liquidity_amount: Uint128::from(0u128), pending_transfer_amount: Uint128::from(0u128), // pending transfer to controller - TODO: how to get hook for transfer finalization? 
        },
        controller_config: ControllerConfig {
            free_amount: Uint128::from(10000u128),
            pending_transfer_amount: Uint128::from(0u128),
            stacked_amount_to_deposit: Uint128::from(0u128),

            // unused fields
            transfer_channel_id: "channel-1".to_string(),deposit_denom: "stake".to_string(), // `ibc/xxxxuatom`
        },        // unused fields
        owner: Addr::unchecked("owner"),unbond_period: 0,total_deposit: Uint128::from(0u128),last_unbonding_id: 1u64,redemption_rate: STAKE_RATE_MULTIPLIER,total_withdrawn: Uint128::from(0u128),transfer_timeout: 300, ica_connection_id: "".to_string(),ica_channel_id: "".to_string(),ica_account: "".to_string(),phase_step: 1u64,pending_icq: 0u64,
    };

    let ica_amounts = determine_ica_amounts(config);

    // NOTE: below nums are just filled in by refering to the code.
    // Of course, this doesn't assure the code itself is designed as intended
    assert_eq!(ica_amounts.to_swap_atom, Uint128::from(5000u128));
    assert_eq!(ica_amounts.to_swap_osmo, Uint128::from(2500u128));
    assert_eq!(ica_amounts.to_add_lp, Uint128::from(9000000000000000000000u128));
    assert_eq!(ica_amounts.to_transfer_to_host, Uint128::from(10000u128));
}

#[test]
fn determine_ica_amounts_for_withdraw() {
    let config = Config {
        phase: Phase::Withdraw,
        host_config: HostConfig {
            lp_redemption_rate: Uint128::from(2u128),
            free_atom_amount: Uint128::from(10000u128),
            free_lp_amount: Uint128::from(10u128),
            free_osmo_amount: Uint128::from(5000u128),

            // unused fields
            chain_id: "test-1".to_string(),pool_id: 1,transfer_channel_id: "channel-1".to_string(),lock_id: 0u64,lp_denom: "gamm/pool/1".to_string(),bonded_lp_amount: Uint128::from(0u128),pending_bond_lp_amount: Uint128::from(0u128),pending_lp_removal_amount: Uint128::from(0u128), osmo_denom: "uosmo".to_string(),pending_swap_to_atom_amount: Uint128::from(0u128), atom_denom: "stake".to_string(),pending_swap_to_osmo_amount: Uint128::from(0u128), pending_add_liquidity_amount: Uint128::from(0u128), pending_transfer_amount: Uint128::from(0u128), // pending transfer to controller - TODO: how to get hook for transfer finalization? 
        },
        controller_config: ControllerConfig {
            free_amount: Uint128::from(10000u128),
            pending_transfer_amount: Uint128::from(0u128),
            stacked_amount_to_deposit: Uint128::from(1000u128),

            // unused fields
            transfer_channel_id: "channel-1".to_string(),deposit_denom: "stake".to_string(), // `ibc/xxxxuatom`
        },        // unused fields
        owner: Addr::unchecked("owner"),unbond_period: 0,total_deposit: Uint128::from(0u128),last_unbonding_id: 1u64,redemption_rate: STAKE_RATE_MULTIPLIER,total_withdrawn: Uint128::from(0u128),transfer_timeout: 300, ica_connection_id: "".to_string(),ica_channel_id: "".to_string(),ica_account: "".to_string(),phase_step: 1u64,pending_icq: 0u64,
    };

    let ica_amounts = determine_ica_amounts(config);

    // NOTE: below nums are just filled in by refering to the code.
    // Of course, this doesn't assure the code itself is designed as intended
    assert_eq!(ica_amounts.to_swap_atom, Uint128::zero());
    assert_eq!(ica_amounts.to_swap_osmo, Uint128::from(10000u128));
    assert_eq!(ica_amounts.to_remove_lp, Uint128::from(10u128));
    assert_eq!(ica_amounts.to_transfer_to_controller, Uint128::from(10000u128));
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
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    config.phase = Phase::Withdraw;
    config.host_config.free_atom_amount = Uint128::from(10000u128);
    CONFIG.save(deps.as_mut().storage, &config).unwrap();

    let res = execute_ibc_transfer_to_controller(deps.as_mut().storage, mock_env());
    assert!(res.is_ok());
    assert_eq!(res.as_ref().unwrap().messages.len(), 1);
}

#[test]
fn test_execute_transfer_to_controller_with_pending_transfer() {
    let mut deps = setup();

    // When is to_transfer_to_controller is zero.
    let res = execute_ibc_transfer_to_controller(deps.as_mut().storage, mock_env());
    assert!(res.is_ok());
    assert_eq!(res.as_ref().unwrap().messages.len(), 0);

    // When is to_transfer_to_controller is not zero.
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    config.phase = Phase::Withdraw;
    config.host_config.free_atom_amount = Uint128::from(10000u128);
    config.controller_config.pending_transfer_amount = Uint128::from(1000u128);
    CONFIG.save(deps.as_mut().storage, &config).unwrap();

    let res = execute_ibc_transfer_to_controller(deps.as_mut().storage, mock_env());
    assert!(res.is_ok());
    assert_eq!(res.as_ref().unwrap().messages.len(), 1);
}

// test of execute_ica_add_and_bond_liquidity
#[test]
fn test_execute_ica_add_and_bond_liquidity() {
    let mut deps = setup();

    // When is to_transfer_to_controller is zero.
    let res = execute_ica_add_and_bond_liquidity(deps.as_mut().storage, mock_env());
    assert!(res.is_ok());
    assert_eq!(res.as_ref().unwrap().messages.len(), 0);

    // When is to_transfer_to_controller is not zero.
    let mut config: Config = th_query(deps.as_ref(), QueryMsg::Config {  });
    config.host_config.free_atom_amount = Uint128::from(100000u128);
    CONFIG.save(deps.as_mut().storage, &config).unwrap();

    let res = execute_ica_add_and_bond_liquidity(deps.as_mut().storage, mock_env());
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

    let mut deps = setup();
    let res = send_ica_tx(
        deps.as_mut().storage,
        mock_env(),
        "test".to_string(),
        vec![join_pool_to_any(msg1.clone()).unwrap()],
    );
    println!("join pool any: {:?}", join_pool_to_any(msg1.clone()).unwrap());

    assert!(res.is_ok());
    assert_eq!(res.as_ref().unwrap().messages.len(), 1);

    // Assume res is your Response<UnunifiMsg> object
    match &res.as_ref().unwrap().messages[0].msg {
        CosmosMsg::Ibc(IbcMsg::SendPacket { data, .. }) => {
            let packet_data: InterchainAccountPacketData = from_binary(&data.clone()).unwrap();
            let cosmos_tx = Any::decode(packet_data.data.as_slice()).unwrap();
            assert!(cosmos_tx.value.is_empty());
            assert_eq!(cosmos_tx.type_url, "\n!/osmosis.gamm.v1beta1.MsgJoinPool\u{12}4\n\u{6}sender\u{10}\u{1}\u{1a}\u{6}100000\"\u{f}\n\u{5}uosmo\u{12}\u{6}100000\"\u{f}\n\u{5}uatom\u{12}\u{6}100000");
        },
        _ => panic!("Unexpected message type"),
    }
}

