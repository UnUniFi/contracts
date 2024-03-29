use std::num::TryFromIntError;
use std::string::FromUtf8Error;
use thiserror::Error;

use cosmwasm_std::StdError;
use cw_utils::PaymentError;

/// Never is a placeholder to ensure we don't return any errors
#[derive(Error, Debug)]
pub enum Never {}

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("Channel doesn't exist: {id}")]
    NoSuchChannel { id: String },

    #[error("Didn't send any funds")]
    NoFunds {},

    #[error("Contract Freezed")]
    ContractFreezed {},

    #[error("Amount larger than 2**64, not supported by ics20 packets")]
    AmountOverflow {},

    #[error("Only supports channel with ibc version ics20-1, got {version}")]
    InvalidIbcVersion { version: String },

    #[error("Only supports unordered channel")]
    OnlyOrderedChannel {},

    #[error("Only allow one channel")]
    OnlyOneChannel {},

    #[error("Insufficient funds to redeem voucher on channel")]
    InsufficientFunds {},

    #[error("Only accepts tokens that originate on this chain, not native tokens of remote chain")]
    NoForeignTokens {},

    #[error("Parsed port from denom ({port}) doesn't match packet")]
    FromOtherPort { port: String },

    #[error("Parsed channel from denom ({channel}) doesn't match packet")]
    FromOtherChannel { channel: String },

    #[error("User cannot close channel")]
    CannotClose {},

    #[error("Got a submessage reply with unknown id: {id}")]
    UnknownReplyId { id: u64 },

    #[error("Only contract admin can do this")]
    Unauthorized,

    #[error("No allowed token")]
    NoAllowedToken {},

    #[error("Execute msg unknown")]
    UnknownRequest {},

    #[error("Lockup account not found")]
    NoLockupAccount,

    #[error("Action needs a default remote denom")]
    NoDefaultDenom {},

    #[error("Lockup account already create")]
    LockupAccountFound,

    #[error("No unlocked tokens available")]
    NoUnlockedTokens {},
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
