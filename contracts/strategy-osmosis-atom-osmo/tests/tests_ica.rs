use cosmwasm_std::testing::mock_env;
use cosmwasm_std::{Uint128, Addr};
// use cosmwasm_std::Overflow;
// use osmosis_std::types::osmosis::epochs::v1beta1::EpochInfo;
use strategy_osmosis::strategy::{Phase, QueryMsg};
use strategy_osmosis_atom_osmo::ica::{determine_ica_amounts, execute_ibc_transfer_to_controller};
use strategy_osmosis_atom_osmo::state::{Config, STAKE_RATE_MULTIPLIER, HostConfig, ControllerConfig, CONFIG};
use crate::helpers::{setup, th_query};
mod helpers;

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
