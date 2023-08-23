use crate::msgs::Phase;
use crate::state::{Config, DepositToken, IcaAmounts, State};
use cosmwasm_std::Uint128;

pub fn determine_ica_amounts(config: Config, state: State) -> IcaAmounts {
    if config.phase == Phase::Withdraw {
        let amount_to_return = state
            .controller_free_amount
            .checked_sub(state.controller_stacked_amount_to_deposit)
            .unwrap_or(Uint128::from(0u128));

        let mut to_swap_amount = state.free_quote_amount;
        if config.deposit_token == DepositToken::Quote {
            to_swap_amount = state.free_base_amount;
        }

        return IcaAmounts {
            to_swap_amount: to_swap_amount,
            to_remove_lp: state.free_lp_amount,
            to_transfer_to_controller: state.free_base_amount,
            to_transfer_to_host: Uint128::from(0u128),
            to_return_amount: amount_to_return,
        };
    } else {
        return IcaAmounts {
            to_swap_amount: Uint128::from(0u128),
            to_remove_lp: Uint128::from(0u128),
            to_transfer_to_controller: Uint128::from(0u128),
            to_transfer_to_host: state.controller_free_amount,
            to_return_amount: Uint128::from(0u128),
        };
    }
}
