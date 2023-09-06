use cosmwasm_std::{DecimalRangeExceeded, OverflowError, StdError};
use cw_utils::PaymentError;
use thiserror::Error;

/// Never is a placeholder to ensure we don't return any errors
#[derive(Error, Debug)]
pub enum Never {}

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("{0}")]
    DecimalRangeExceeded(#[from] DecimalRangeExceeded),

    #[error("Only gov authority address can do this")]
    Unauthorized,

    #[error("Not supported denom")]
    InvalidDenom,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Invalid LP fee weight")]
    InvalidLpFeeWeight,

    #[error("Insufficient funds for minimum fee")]
    InsufficientFundsForMinFee,
}
