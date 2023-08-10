use crate::msgs::Phase;
use crate::state::{Config, IcaAmounts};
use cosmwasm_std::Uint128;

pub fn determine_ica_amounts(config: Config) -> IcaAmounts {
    if config.phase == Phase::Withdraw {
        let amount_to_return = config
            .controller_config
            .free_amount
            .checked_sub(config.controller_config.stacked_amount_to_deposit)
            .unwrap_or(Uint128::from(0u128));

        return IcaAmounts {
            to_swap_base: Uint128::from(0u128),
            to_swap_quote: config.host_config.free_base_amount,
            to_remove_lp: config.host_config.free_lp_amount,
            to_transfer_to_controller: config.host_config.free_base_amount,
            to_transfer_to_host: Uint128::from(0u128),
            to_return_amount: amount_to_return,
        };
    } else {
        return IcaAmounts {
            to_swap_base: config.host_config.free_base_amount / Uint128::from(2u128),
            to_swap_quote: config.host_config.free_quote_amount / Uint128::from(2u128),
            to_remove_lp: Uint128::from(0u128),
            to_transfer_to_controller: Uint128::from(0u128),
            to_transfer_to_host: config.controller_config.free_amount,
            to_return_amount: Uint128::from(0u128),
        };
    }
}
