use std::{collections::HashMap, convert::TryInto};

use candid::{Nat, Principal};
use cap_sdk::{DetailValue, IndefiniteEvent, IntoEvent, TryFromEvent};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug)]
pub struct XTCTransactionDetailsERC20 {
    to: Principal,
    amount: Nat,
    fee: Nat,
    index: Nat,
}

#[derive(Debug, Error)]
pub enum XTCTransactionDetailsERC20Error {
    #[error("missing key {0}")]
    MissingKey(String),
    #[error("couldn't convert item with key {0} to DetailValue")]
    ConversionError(String),
    #[error("invalid operation {0}")]
    InvalidOperation(String),
}

#[cfg(feature = "sdk-impls")]
impl IntoEvent for XTCTransactionDetailsERC20 {
    fn details(&self) -> Vec<(String, DetailValue)> {
        vec![
            ("to".into(), self.to.into()),
            ("amount".into(), self.amount.clone().into()),
            ("fee".into(), self.fee.clone().into()),
            ("index".into(), self.index.clone().into()),
        ]
    }
}

#[cfg(feature = "sdk-impls")]
trait MapFailed<T, E> {
    fn map_failure(self, key: &'static str) -> Result<T, E>;
}

#[cfg(feature = "sdk-impls")]
impl<T, O> MapFailed<T, XTCTransactionDetailsERC20Error> for Result<T, O> {
    fn map_failure(self, key: &'static str) -> Result<T, XTCTransactionDetailsERC20Error> {
        self.map_err(|_| XTCTransactionDetailsERC20Error::ConversionError(key.to_owned()))
    }
}

#[cfg(feature = "sdk-impls")]
fn try_get_and_clone(
    map: &HashMap<String, DetailValue>,
    key: &'static str,
) -> Result<DetailValue, XTCTransactionDetailsERC20Error> {
    if let Some(item) = map.get(key) {
        Ok(item.clone())
    } else {
        Err(XTCTransactionDetailsERC20Error::MissingKey(key.to_owned()))
    }
}

#[cfg(feature = "sdk-impls")]
impl TryFromEvent for XTCTransactionDetailsERC20 {
    type Error = XTCTransactionDetailsERC20Error;

    fn try_from_event(
        event: impl Into<IndefiniteEvent>,
    ) -> Result<Self, XTCTransactionDetailsERC20Error> {
        let details = event.into().details;

        let map = details.iter().cloned().collect::<HashMap<_, _>>();

        Ok(Self {
            to: try_get_and_clone(&map, "to")?
                .try_into()
                .map_failure("to")?,
            amount: try_get_and_clone(&map, "amount")?
                .try_into()
                .map_failure("amount")?,
            fee: try_get_and_clone(&map, "fee")?
                .try_into()
                .map_failure("fee")?,
            index: try_get_and_clone(&map, "index")?
                .try_into()
                .map_failure("index")?,
        })
    }
}

pub struct XTCTransactionDetailsLegacy {
    pub fee: Nat,
    pub cycles: Nat,
    pub kind: XTCTransactionKindLegacy,
}

#[cfg(feature = "sdk-impls")]
impl IntoEvent for XTCTransactionKindLegacy {
    fn operation(&self) -> Option<&'static str> {
        Some(match *self {
            Self::Transfer { .. } => "transfer",
            Self::TransferFrom { .. } => "transfer_from",
            Self::Approve { .. } => "approve",
            Self::Burn { .. } => "burn",
            Self::Mint { .. } => "mint",
            Self::CanisterCalled { .. } => "canister_called",
            Self::CanisterCreated { .. } => "canister_created",
        })
    }

    fn details(&self) -> Vec<(String, DetailValue)> {
        match self {
            Self::Transfer { from, to } => {
                vec![
                    ("to".to_owned(), to.clone().into()),
                    ("from".to_owned(), from.clone().into()),
                ]
            }
            Self::TransferFrom { from, to } => {
                vec![
                    ("to".to_owned(), to.clone().into()),
                    ("from".to_owned(), from.clone().into()),
                ]
            }
            Self::Approve { from, to } => {
                vec![
                    ("to".to_owned(), to.clone().into()),
                    ("from".to_owned(), from.clone().into()),
                ]
            }
            Self::Burn { from, to } => {
                vec![
                    ("to".to_owned(), to.clone().into()),
                    ("from".to_owned(), from.clone().into()),
                ]
            }
            Self::Mint { to } => {
                vec![("to".to_owned(), to.clone().into())]
            }
            Self::CanisterCalled { from, to, method } => {
                vec![
                    ("to".to_owned(), to.clone().into()),
                    ("from".to_owned(), from.clone().into()),
                    ("method".to_owned(), method.clone().into()),
                ]
            }
            Self::CanisterCreated { from, canister } => {
                vec![
                    ("canister".to_owned(), canister.clone().into()),
                    ("from".to_owned(), from.clone().into()),
                ]
            }
        }
    }
}

#[cfg(feature = "sdk-impls")]
impl TryFromEvent for XTCTransactionKindLegacy {
    type Error = XTCTransactionDetailsERC20Error;

    fn try_from_event(event: impl Into<IndefiniteEvent>) -> Result<Self, Self::Error> {
        let event = event.into();

        let map = event.details.iter().cloned().collect::<HashMap<_, _>>();

        Ok(match event.operation.as_str() {
            "transfer" => XTCTransactionKindLegacy::Transfer {
                to: try_get_and_clone(&map, "to")?
                    .try_into()
                    .map_failure("to")?,
                from: try_get_and_clone(&map, "from")?
                    .try_into()
                    .map_failure("from")?,
            },
            "transfer_from" => XTCTransactionKindLegacy::TransferFrom {
                to: try_get_and_clone(&map, "to")?
                    .try_into()
                    .map_failure("to")?,
                from: try_get_and_clone(&map, "from")?
                    .try_into()
                    .map_failure("from")?,
            },
            "approve" => XTCTransactionKindLegacy::Approve {
                to: try_get_and_clone(&map, "to")?
                    .try_into()
                    .map_failure("to")?,
                from: try_get_and_clone(&map, "from")?
                    .try_into()
                    .map_failure("from")?,
            },
            "burn" => XTCTransactionKindLegacy::Burn {
                to: try_get_and_clone(&map, "to")?
                    .try_into()
                    .map_failure("to")?,
                from: try_get_and_clone(&map, "from")?
                    .try_into()
                    .map_failure("from")?,
            },
            "mint" => XTCTransactionKindLegacy::Mint {
                to: try_get_and_clone(&map, "to")?
                    .try_into()
                    .map_failure("to")?,
            },
            "canister_called" => XTCTransactionKindLegacy::CanisterCalled {
                method: try_get_and_clone(&map, "method")?
                    .try_into()
                    .map_failure("method")?,
                to: try_get_and_clone(&map, "to")?
                    .try_into()
                    .map_failure("to")?,
                from: try_get_and_clone(&map, "from")?
                    .try_into()
                    .map_failure("from")?,
            },
            "canister_created" => XTCTransactionKindLegacy::CanisterCreated {
                canister: try_get_and_clone(&map, "canister")?
                    .try_into()
                    .map_failure("canister")?,
                from: try_get_and_clone(&map, "from")?
                    .try_into()
                    .map_failure("from")?,
            },
            operation => {
                return Err(XTCTransactionDetailsERC20Error::InvalidOperation(
                    operation.to_owned(),
                ))
            }
        })
    }
}

#[derive(Serialize, Deserialize)]
pub enum XTCTransactionKindLegacy {
    Transfer {
        from: Principal,
        to: Principal,
    },

    Mint {
        to: Principal,
    },

    Burn {
        from: Principal,
        to: Principal,
    },

    CanisterCalled {
        from: Principal,
        to: Principal,
        method: String,
    },

    CanisterCreated {
        from: Principal,
        canister: Principal,
    },

    TransferFrom {
        from: Principal,
        to: Principal,
    },

    Approve {
        from: Principal,
        to: Principal,
    },
}
