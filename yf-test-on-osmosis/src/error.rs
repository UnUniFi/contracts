use cosmwasm_std::{Coin, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Insufficient Fund")]
    InsufficientFund { coin: Coin },

    #[error("Invalid Pool Route: {reason:?}")]
    InvalidPoolRoute { reason: String },

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("Failed SwapJoin: {reason:?}")]
    FailedSwapJoin { reason: String },

    #[error("Failed ExitSwap: {reason:?}")]
    FailedExitSwap { reason: String },

    #[error("No share for the sender: {sender:?}")]
    NoShareForSender { sender: String },

    #[error("Sender's share_in_amont is insufficient. Actual share amount is: {share_amount:?}")]
    InsufficientShareAmount { share_amount: String },

    #[error("Invalid deposit denom: {true_denom:?}")]
    InvalidDepositDenom { true_denom: String },
}
