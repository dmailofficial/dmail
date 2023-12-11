use std::convert::TryFrom;

use candid::{CandidType, Int, Nat, Principal};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(feature = "sdk-impls")]
pub mod cap;

#[derive(CandidType, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TransactionStatus {
    Succeeded,
    Failed,
}

impl<'a> TryFrom<&'a str> for TransactionStatus {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(match value {
            "succeeded" => Self::Succeeded,
            "failed" => Self::Failed,
            _ => return Err(()),
        })
    }
}

impl Into<&'static str> for TransactionStatus {
    fn into(self) -> &'static str {
        match self {
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
        }
    }
}

impl TransactionStatus {
    pub fn into_str(self) -> &'static str {
        self.into()
    }
}

#[derive(CandidType, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Operation {
    Approve,
    Mint,
    Transfer,
    TransferFrom,
    Burn,
    #[cfg(feature = "alpha-dip20-dank")]
    CanisterCalled,
    #[cfg(feature = "alpha-dip20-dank")]
    CanisterCreated,
}

impl<'a> TryFrom<&'a str> for Operation {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(match value {
            "approve" => Self::Approve,
            "mint" => Self::Mint,
            "transfer" => Self::Transfer,
            "transfer_from" => Self::TransferFrom,
            "burn" => Self::Burn,
            #[cfg(feature = "alpha-dip20-dank")]
            "canister_called" => Self::CanisterCalled,
            #[cfg(feature = "alpha-dip20-dank")]
            "canister_created" => Self::CanisterCreated,
            _ => return Err(()),
        })
    }
}

impl Into<&'static str> for Operation {
    fn into(self) -> &'static str {
        match self {
            Self::Approve => "approve",
            Self::Mint => "mint",
            Self::Transfer => "transfer",
            Self::TransferFrom => "transfer_from",
            Self::Burn => "burn",
            #[cfg(feature = "alpha-dip20-dank")]
            Self::CanisterCalled => "canister_called",
            #[cfg(feature = "alpha-dip20-dank")]
            Self::CanisterCreated => "canister_created",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TxRecord {
    pub caller: Option<Principal>,
    pub index: Nat,
    pub from: Principal,
    pub to: Principal,
    pub amount: Nat,
    pub fee: Nat,
    pub timestamp: Int,
    pub status: TransactionStatus,
    pub operation: Operation,
}

#[derive(Debug, Error)]
pub enum DIP20ParseError {
    #[error("missing key {0}")]
    MissingKey(String),
    #[error("couldn't convert item with key {0} to DetailValue")]
    ConversionError(String),
    #[error("invalid operation {0}")]
    InvalidOperation(String),
}
