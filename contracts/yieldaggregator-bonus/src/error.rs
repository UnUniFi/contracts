use cosmwasm_std::{DecimalRangeExceeded, OverflowError, StdError};
use cw_utils::PaymentError;
use std::string::FromUtf8Error;
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

    #[error("Insufficient funds to redeem voucher on channel")]
    InsufficientFunds {},

    #[error("Only contract admin can do this")]
    Unauthorized,

    #[error("No allowed token")]
    NoAllowedToken {},

    #[error("Not started or ended")]
    InvalidBonusWindowPeriod {},

    #[error("Already staked")]
    AlreadyStaked {},

    #[error("Insufficient budget to register bonus window")]
    InsufficientBudget {},

    #[error("Bonus window not ended yet")]
    BonusWindowNotEndedYet
}

impl From<FromUtf8Error> for ContractError {
    fn from(_: FromUtf8Error) -> Self {
        ContractError::Std(StdError::invalid_utf8("parsing denom key"))
    }
}

impl From<NoDeposit> for StdError {
    fn from(_: NoDeposit) -> Self {
        StdError::generic_err("No deposit")
    }
}

pub struct NoDeposit {}
