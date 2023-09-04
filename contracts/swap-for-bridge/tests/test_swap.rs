use crate::helpers::setup;
use cosmwasm_std::{
    testing::{mock_env, mock_info},
    Coin,
};
use swap_for_bridge::{execute::swap::execute_swap, msgs::SwapMsg};
mod helpers;

#[test]
fn swap() {
    let mut deps = setup();

    let sender = "anyone";
    // Change with other values for further tests
    execute_swap(
        deps.as_mut(),
        mock_env(),
        mock_info(sender, &[Coin::new(100, "denom1")]),
        SwapMsg {
            output_denom: "denom2".to_string(),
            recipient: None,
        },
    )
    .unwrap();
}
