use cosmwasm_std::{DecimalRangeExceeded, OverflowError, StdError};
use cw_utils::PaymentError;
use std::num::TryFromIntError;
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

    #[error("Amount larger than 2**64, not supported by ics20 packets")]
    AmountOverflow {},

    #[error("Insufficient funds to redeem voucher on channel")]
    InsufficientFunds {},

    #[error("Only contract admin can do this")]
    Unauthorized,

    #[error("Mismatching bid denom")]
    BidDenomMismatch {},

    #[error("Bid already exists")]
    BidAlreadyExists {},

    #[error("Execute msg unknown")]
    UnknownRequest {},

    #[error("Maximum address length")]
    MaxAddrLength {},
}

impl From<FromUtf8Error> for ContractError {
    fn from(_: FromUtf8Error) -> Self {
        ContractError::Std(StdError::invalid_utf8("parsing denom key"))
    }
}

impl From<TryFromIntError> for ContractError {
    fn from(_: TryFromIntError) -> Self {
        ContractError::AmountOverflow {}
    }
}

impl From<NoDeposit> for StdError {
    fn from(_: NoDeposit) -> Self {
        StdError::generic_err("No deposit")
    }
}

pub struct NoDeposit {}
