use cosmwasm_std::{Addr, Coin, Decimal, Deps, Timestamp, Uint128};
use osmosis_std::shim::Timestamp as OsmosisTimestamp;
use osmosis_std::types::osmosis::gamm::v1beta1::{
    MsgExitSwapShareAmountIn, MsgJoinSwapExternAmountIn, QueryTotalPoolLiquidityRequest,
    SwapAmountInRoute,
};
use osmosis_std::types::osmosis::twap::v1beta1::TwapQuerier;

use crate::error::ContractError;

pub fn generate_join_swap_extern_msg(
    // deps: Deps,
    sender: Addr,
    pool_id: u64,
    token_in: Coin,
    share_out_min_amount: String,
) -> Result<MsgJoinSwapExternAmountIn, ContractError> {
    // get trade route
    dbg!("generating");

    Ok(MsgJoinSwapExternAmountIn {
        sender: sender.into_string(),
        pool_id: pool_id,
        token_in: Some(token_in.into()),
        share_out_min_amount: share_out_min_amount,
    })
}

pub fn generate_exit_swap_share_amount_in(
    // deps: Deps,
    sender: Addr,
    pool_id: u64,
    token_out_denom: String,
    share_amount_in: String,
    token_out_min_amount: String,
) -> Result<MsgExitSwapShareAmountIn, ContractError> {
    dbg!("generating");

    Ok(MsgExitSwapShareAmountIn {
        sender: sender.into(),
        pool_id: pool_id,
        token_out_denom: token_out_denom,
        share_in_amount: share_amount_in,
        token_out_min_amount: token_out_min_amount,
    })
}

// pub fn calculate_min_output_from_twap(
//     deps: Deps,
//     input_token: Coin,
//     output_denom: String,
//     now: Timestamp,
//     percentage_impact: Decimal,
// ) -> Result<Coin, ContractError> {
//     // get trade route
//     let route = ROUTING_TABLE.load(deps.storage, (&input_token.denom, &output_denom))?;
//     if route.is_empty() {
//         return Err(ContractError::InvalidPoolRoute {
//             reason: format!("No route foung for {} -> {output_denom}", input_token.denom),
//         });
//     }

//     let percentage = percentage_impact.div(Uint128::new(100));

//     let mut twap_price: Decimal = Decimal::one();

//     // When swapping from input to output, we need to quote the price in the input token
//     // For example when seling osmo to buy atom:
//     //  price of <out> is X<in> (i.e.: price of atom is Xosmo)
//     let mut quote_denom = input_token.denom;

//     let start_time = now.minus_seconds(1);
//     let start_time = OsmosisTimestamp {
//         seconds: start_time.seconds() as i64,
//         nanos: 0_i32,
//     };

//     //deps.api.debug(&format!("twap_price: {twap_price}"));

//     for route_part in route {
//         //deps.api.debug(&format!("route part: {route_part:?}"));

//         let twap = TwapQuerier::new(&deps.querier)
//             .arithmetic_twap_to_now(
//                 route_part.pool_id,
//                 route_part.token_out_denom.clone(), // base_asset
//                 quote_denom.clone(),                // quote_asset
//                 Some(start_time.clone()),
//             )?
//             .arithmetic_twap;

//         //deps.api.debug(&format!("twap = {twap}"));

//         let twap: Decimal = twap.parse().map_err(|_e| ContractError::CustomError {
//             val: "Invalid twap value received from the chain".to_string(),
//         })?;

//         twap_price =
//             twap_price
//                 .checked_mul(twap.into())
//                 .map_err(|_e| ContractError::CustomError {
//                     val: format!("Invalid value for twap price: {twap_price} * {twap}"),
//                 })?;

//         // the current output is the input for the next route_part
//         quote_denom = route_part.token_out_denom;
//         //deps.api.debug(&format!("twap_price: {twap_price}"));
//     }

//     twap_price = twap_price - twap_price.mul(percentage);
//     deps.api.debug(&format!(
//         "twap_price minus {percentage_impact}%: {twap_price}"
//     ));

//     let min_out: Uint128 = input_token.amount.mul(twap_price);
//     deps.api.debug(&format!("min: {min_out}"));

//     Ok(Coin::new(min_out.into(), output_denom))
// }
