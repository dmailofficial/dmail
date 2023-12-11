use std::{collections::HashMap, convert::TryInto};

use candid::{Int, Nat, Principal};
use cap_sdk::{DetailValue, DetailsBuilder, IndefiniteEvent, IntoEvent, TryFromEvent, TypedEvent};
use num_bigint::{BigInt, BigUint};

use super::{DIP20ParseError, Operation, TransactionStatus, TxRecord};

/// DIP20 Details for a [`TypedEvent`] or [`TypedIndefiniteEvent`][cap_sdk::TypedIndefiniteEvent].
///
/// # A note on `caller`.
/// Cap's `caller` is **not** optional. Unlike DIP20. Caller can be determined
/// from the transaction type, so it is automatically populated based on the type
/// of operation.
///
/// # Examples
///
/// ### Attempting to convert a Cap record into a `TypedEvent<DIP20Details>`.
///
/// Also demonstrates acquiring additional information from the event,
/// such as the caller and the timestamp with an API similar to how [`TxRecord`]
/// stores these parameters.
///
/// ```rust
/// // Retrieve a transaction from cap. Since this contract uses the
/// // DIP20 standard we know its DIP20 compliant and will unwrap the
/// // conversion.
/// let transaction = get_transaction(230948).unwrap();
///
/// let typed_transaction: TypedEvent<DIP20Details> = transaction.try_into().unwrap();
///
/// // Some utility methods are included in an "Extension Trait". Which
/// // is a pattern which allows extending a type while following Rust's
/// // orphaning rules.
/// use cap_standards::dip20::DIP20EventExt;
///
/// // Accessing data within the event is easy since it's just a structural
/// // enum. This isn't demonstrated here because it's slightly bulky for an
/// // example. However, you can access the caller and timestamp with the extension
/// // trait.
/// let caller = typed_transaction.caller();
/// let timestamp = typed_transaction.timestamp();
/// ```
///
/// ### Converting a `TypedEvent<DIP20Details>` to a TxRecord.
/// ```rust
/// let event = TypedEvent {
///     caller: ic::id(),
///     time: 7270727,
///     details: DIP20Details::Mint {
///         from: Principal::from_text("aaaaa-aa").unwrap(),
///         to: ic::id(),
///         amount: BigUint(23089).into(),
///         fee: BigUint(0).into(),
///         status: TransactionStatus::Succeeded
///     }
/// };
///
/// let tx_record: TxRecord = event.into();
/// ```
///
#[derive(Clone)]
pub enum DIP20Details {
    /// Indicates that `from` (the account owner) has approved `to` (the spender) to withdraw
    /// tokens from the account up to `amount` amount.
    Approve {
        /// The authorizer.
        ///
        /// Should be the same as its cap event's `caller`.
        from: Principal,
        to: Principal,
        amount: Nat,
        fee: Nat,
        status: TransactionStatus,
    },
    /// Indicates that `amount` number of new tokens have been minted
    /// to user `to`.
    Mint {
        /// The executor. The spec defines this as only the owner of the canister.
        ///
        /// Should be the same as its cap event's `caller`.
        from: Principal,
        to: Principal,
        amount: Nat,
        fee: Nat,
        status: TransactionStatus,
    },
    /// Indicates that `value` tokens have been transfered to user `to` from account `from`.
    /// On a Cap Event, `caller` is `from`.
    Transfer {
        /// Should be the same as its cap event's `caller` as this can only be executed
        /// on the account of the caller.
        from: Principal,
        to: Principal,
        amount: Nat,
        fee: Nat,
        status: TransactionStatus,
    },
    /// Indicates that a third-party has transferred tokens from user `from` to user `to`.
    ///
    /// On a Cap Event, `caller` isn't always `from`.
    ///
    /// Approval to execute this action should come from an `Approval` event.
    TransferFrom {
        from: Principal,
        to: Principal,
        amount: Nat,
        fee: Nat,
        status: TransactionStatus,
    },
    Burn {
        from: Principal,
        to: Principal,
        amount: Nat,
        fee: Nat,
        status: TransactionStatus,
    },
    #[cfg(feature = "alpha-dip20-dank")]
    CanisterCalled {
        from: Principal,
        to: Principal,
        method_name: String,
        amount: Nat,
        fee: Nat,
        status: TransactionStatus,
    },
    #[cfg(feature = "alpha-dip20-dank")]
    CanisterCreated {
        from: Principal,
        canister: Principal,
        amount: Nat,
        fee: Nat,
        status: TransactionStatus,
    },
}

pub trait DIP20EventExt {
    fn timestamp(&self) -> Int;
    fn caller(&self) -> Principal;

    fn into_txrecord(self) -> TxRecord;
}

impl DIP20EventExt for TypedEvent<DIP20Details> {
    fn timestamp(&self) -> Int {
        Int(BigInt::default() + self.time)
    }

    fn caller(&self) -> Principal {
        self.caller
    }

    fn into_txrecord(self) -> TxRecord {
        match self.details {
            DIP20Details::Approve {
                from: owner,
                to: spender,
                amount: limit,
                fee,
                status,
            } => TxRecord {
                caller: Some(self.caller),
                timestamp: Int(BigInt::default() + self.time),
                index: Nat(BigUint::default()),
                from: owner,
                to: spender,
                amount: limit,
                fee,
                status,
                operation: Operation::Approve,
            },
            DIP20Details::Mint {
                from,
                to,
                amount,
                fee,
                status,
            } => TxRecord {
                caller: Some(self.caller),
                timestamp: Int(BigInt::default() + self.time),
                index: Nat(BigUint::default()),
                from,
                to,
                amount,
                fee,
                status,
                operation: Operation::Mint,
            },
            DIP20Details::Transfer {
                from,
                to,
                amount,
                fee,
                status,
            } => TxRecord {
                caller: Some(self.caller),
                timestamp: Int(BigInt::default() + self.time),
                index: Nat(BigUint::default()),
                from,
                to,
                amount,
                fee,
                status,
                operation: Operation::Transfer,
            },
            DIP20Details::TransferFrom {
                from,
                to,
                amount,
                fee,
                status,
            } => TxRecord {
                caller: Some(self.caller),
                timestamp: Int(BigInt::default() + self.time),
                index: Nat(BigUint::default()),
                from,
                to,
                amount,
                fee,
                status,
                operation: Operation::TransferFrom,
            },
            DIP20Details::Burn {
                from,
                to,
                amount,
                fee,
                status,
            } => TxRecord {
                caller: None,
                timestamp: Int(BigInt::default() + self.time),
                from,
                to,
                amount,
                fee,
                operation: Operation::Burn,
                status,
                index: Nat(BigUint::default()),
            },
            #[cfg(feature = "alpha-dip20-dank")]
            DIP20Details::CanisterCalled {
                from,
                to,
                amount,
                fee,
                status,
                ..
            } => TxRecord {
                caller: None,
                timestamp: Int(BigInt::default() + self.time),
                from,
                to,
                amount,
                fee,
                operation: Operation::Burn,
                status,
                index: Nat(BigUint::default()),
            },
            #[cfg(feature = "alpha-dip20-dank")]
            DIP20Details::CanisterCreated {
                from,
                canister,
                amount,
                fee,
                status,
            } => TxRecord {
                caller: None,
                timestamp: Int(BigInt::default() + self.time),
                from,
                to: canister,
                amount,
                fee,
                operation: Operation::Burn,
                status,
                index: Nat(BigUint::default()),
            },
        }
    }
}

impl IntoEvent for DIP20Details {
    fn operation(&self) -> Option<&'static str> {
        Some(match self {
            Self::Approve { .. } => "approve",
            Self::Mint { .. } => "mint",
            Self::Transfer { .. } => "transfer",
            Self::TransferFrom { .. } => "transfer_from",
            Self::Burn { .. } => "burn",
            #[cfg(feature = "alpha-dip20-dank")]
            Self::CanisterCalled { .. } => "canister_called",
            #[cfg(feature = "alpha-dip20-dank")]
            Self::CanisterCreated { .. } => "canister_created",
        })
    }

    fn details(&self) -> Vec<(String, cap_sdk::DetailValue)> {
        match self {
            Self::Approve {
                from: owner,
                to: spender,
                amount: limit,
                fee,
                status,
            } => {
                let status_string = status.into_str().to_owned();

                DetailsBuilder::default()
                    .insert("from", owner.clone())
                    .insert("to", spender.clone())
                    .insert("amount", limit.clone())
                    .insert("fee", fee.clone())
                    .insert("status", status_string.clone())
                    .build()
            }
            Self::Mint {
                from,
                to,
                amount,
                fee,
                status,
            } => {
                let status_string = status.into_str().to_owned();

                DetailsBuilder::default()
                    .insert("from", from.clone())
                    .insert("to", to.clone())
                    .insert("amount", amount.clone())
                    .insert("fee", fee.clone())
                    .insert("status", status_string.clone())
                    .build()
            }
            Self::Transfer {
                from,
                to,
                amount,
                fee,
                status,
            } => {
                let status_string = status.into_str().to_owned();

                DetailsBuilder::default()
                    .insert("from", from.clone())
                    .insert("to", to.clone())
                    .insert("amount", amount.clone())
                    .insert("fee", fee.clone())
                    .insert("status", status_string.clone())
                    .build()
            }
            Self::TransferFrom {
                from,
                to,
                amount,
                fee,
                status,
            } => {
                let status_string = status.into_str().to_owned();

                DetailsBuilder::default()
                    .insert("from", from.clone())
                    .insert("to", to.clone())
                    .insert("amount", amount.clone())
                    .insert("fee", fee.clone())
                    .insert("status", status_string.clone())
                    .build()
            }
            Self::Burn {
                from,
                to,
                amount,
                fee,
                status,
            } => {
                let status_string = status.into_str().to_owned();

                DetailsBuilder::default()
                    .insert("from", from.clone())
                    .insert("to", to.clone())
                    .insert("amount", amount.clone())
                    .insert("fee", fee.clone())
                    .insert("status", status_string.clone())
                    .build()
            }
            #[cfg(feature = "alpha-dip20-dank")]
            Self::CanisterCalled {
                from,
                to,
                amount,
                fee,
                status,
                method_name,
            } => {
                let status_string = status.into_str().to_owned();

                DetailsBuilder::default()
                    .insert("from", from.clone())
                    .insert("to", to.clone())
                    .insert("amount", amount.clone())
                    .insert("fee", fee.clone())
                    .insert("status", status_string.clone())
                    .insert("method_name", method_name.clone())
                    .build()
            }
            #[cfg(feature = "alpha-dip20-dank")]
            Self::CanisterCreated {
                from,
                canister,
                amount,
                fee,
                status,
            } => {
                let status_string = status.into_str().to_owned();

                DetailsBuilder::default()
                    .insert("from", from.clone())
                    .insert("to", canister.clone())
                    .insert("amount", amount.clone())
                    .insert("fee", fee.clone())
                    .insert("status", status_string.clone())
                    .build()
            }
        }
    }
}

impl TryFromEvent for DIP20Details {
    type Error = DIP20ParseError;

    fn try_from_event(event: impl Into<IndefiniteEvent>) -> Result<Self, Self::Error> {
        let event = event.into();

        let details = event.details.iter().cloned().collect::<HashMap<_, _>>();

        Ok(match event.operation.as_str() {
            "approve" => {
                let status_string: String = details
                    .get_detail("status")?
                    .try_into()
                    .map_failure("status")?;

                Self::Approve {
                    from: details.get_detail("from")?.try_into().map_failure("from")?,
                    to: details.get_detail("to")?.try_into().map_failure("to")?,
                    amount: details
                        .get_detail("amount")?
                        .try_into()
                        .map_failure("amount")?,
                    fee: details.get_detail("fee")?.try_into().map_failure("fee")?,
                    status: status_string.as_str().try_into().map_failure("status")?,
                }
            }
            "mint" => {
                let status_string: String = details
                    .get_detail("status")?
                    .try_into()
                    .map_failure("status")?;

                Self::Mint {
                    from: details.get_detail("from")?.try_into().map_failure("from")?,
                    to: details.get_detail("to")?.try_into().map_failure("to")?,
                    amount: details
                        .get_detail("amount")?
                        .try_into()
                        .map_failure("amount")?,
                    fee: details.get_detail("fee")?.try_into().map_failure("fee")?,
                    status: status_string.as_str().try_into().map_failure("status")?,
                }
            }
            "transfer" => {
                let status_string: String = details
                    .get_detail("status")?
                    .try_into()
                    .map_failure("status")?;

                Self::Transfer {
                    from: details.get_detail("from")?.try_into().map_failure("from")?,
                    to: details.get_detail("to")?.try_into().map_failure("to")?,
                    amount: details
                        .get_detail("amount")?
                        .try_into()
                        .map_failure("amount")?,
                    fee: details.get_detail("fee")?.try_into().map_failure("fee")?,
                    status: status_string.as_str().try_into().map_failure("status")?,
                }
            }
            "transfer_from" => {
                let status_string: String = details
                    .get_detail("status")?
                    .try_into()
                    .map_failure("status")?;

                Self::TransferFrom {
                    from: details.get_detail("from")?.try_into().map_failure("from")?,
                    to: details.get_detail("to")?.try_into().map_failure("to")?,
                    amount: details
                        .get_detail("amount")?
                        .try_into()
                        .map_failure("amount")?,
                    fee: details.get_detail("fee")?.try_into().map_failure("fee")?,
                    status: status_string.as_str().try_into().map_failure("status")?,
                }
            }

            "burn" => {
                let status_string: String = details
                    .get_detail("status")?
                    .try_into()
                    .map_failure("status")?;

                Self::Burn {
                    from: details.get_detail("from")?.try_into().map_failure("from")?,
                    to: details.get_detail("to")?.try_into().map_failure("to")?,
                    amount: details
                        .get_detail("amount")?
                        .try_into()
                        .map_failure("amount")?,
                    fee: details.get_detail("fee")?.try_into().map_failure("fee")?,
                    status: status_string.as_str().try_into().map_failure("status")?,
                }
            }
            #[cfg(feature = "alpha-dip20-dank")]
            "canister_called" => {
                let status_string: String = details
                    .get_detail("status")?
                    .try_into()
                    .map_failure("status")?;

                Self::CanisterCalled {
                    from: details.get_detail("from")?.try_into().map_failure("from")?,
                    to: details.get_detail("to")?.try_into().map_failure("to")?,
                    amount: details
                        .get_detail("amount")?
                        .try_into()
                        .map_failure("amount")?,
                    fee: details.get_detail("fee")?.try_into().map_failure("fee")?,
                    status: status_string.as_str().try_into().map_failure("status")?,
                    method_name: details
                        .get_detail("method_name")?
                        .try_into()
                        .map_failure("method_name")?,
                }
            }
            #[cfg(feature = "alpha-dip20-dank")]
            "canister_created" => {
                let status_string: String = details
                    .get_detail("status")?
                    .try_into()
                    .map_failure("status")?;

                Self::CanisterCreated {
                    from: details.get_detail("from")?.try_into().map_failure("from")?,
                    canister: details.get_detail("to")?.try_into().map_failure("to")?,
                    amount: details
                        .get_detail("amount")?
                        .try_into()
                        .map_failure("amount")?,
                    fee: details.get_detail("fee")?.try_into().map_failure("fee")?,
                    status: status_string.as_str().try_into().map_failure("status")?,
                }
            }
            operation => return Err(DIP20ParseError::InvalidOperation(operation.to_owned())),
        })
    }
}

impl Into<TypedEvent<DIP20Details>> for TxRecord {
    fn into(self) -> TypedEvent<DIP20Details> {
        match self.operation {
            Operation::Approve => {
                let time = self.timestamp.0.try_into().unwrap();

                TypedEvent {
                    caller: self.from,
                    time,
                    details: DIP20Details::Approve {
                        from: self.from,
                        to: self.to,
                        amount: self.amount,
                        fee: self.fee,
                        status: self.status,
                    },
                }
            }
            Operation::Mint => {
                let time: u64 = self.timestamp.0.try_into().unwrap();

                TypedEvent {
                    caller: self.caller.unwrap(),
                    time,
                    details: DIP20Details::Mint {
                        from: self.from,
                        to: self.to,
                        amount: self.amount,
                        fee: self.fee,
                        status: self.status,
                    },
                }
            }
            Operation::Transfer => {
                let time: u64 = self.timestamp.0.try_into().unwrap();

                TypedEvent {
                    caller: self.from,
                    time,
                    details: DIP20Details::Transfer {
                        from: self.from,
                        to: self.to,
                        amount: self.amount,
                        fee: self.fee,
                        status: self.status,
                    },
                }
            }
            Operation::TransferFrom => {
                let time: u64 = self.timestamp.0.try_into().unwrap();

                TypedEvent {
                    caller: self.caller.unwrap(),
                    time,
                    details: DIP20Details::TransferFrom {
                        from: self.from,
                        to: self.to,
                        amount: self.amount,
                        fee: self.fee,
                        status: self.status,
                    },
                }
            }
            Operation::Burn => {
                let time: u64 = self.timestamp.0.try_into().unwrap();

                TypedEvent {
                    caller: self.caller.unwrap(),
                    time,
                    details: DIP20Details::Burn {
                        from: self.from,
                        to: self.to,
                        amount: self.amount,
                        fee: self.fee,
                        status: self.status,
                    },
                }
            }
            #[cfg(feature = "alpha-dip20-dank")]
            Operation::CanisterCalled => {
                panic!("Invalid conversion.");
            }
            #[cfg(feature = "alpha-dip20-dank")]
            Operation::CanisterCreated => {
                let time: u64 = self.timestamp.0.try_into().unwrap();

                TypedEvent {
                    caller: self.caller.unwrap(),
                    time,
                    details: DIP20Details::CanisterCreated {
                        from: self.from,
                        canister: self.to,
                        amount: self.amount,
                        fee: self.fee,
                        status: self.status,
                    },
                }
            }
        }
    }
}
trait GetDetailExt {
    fn get_detail(&self, detail: &'static str) -> Result<DetailValue, DIP20ParseError>;
}

impl GetDetailExt for HashMap<String, DetailValue> {
    fn get_detail(&self, detail: &'static str) -> Result<DetailValue, DIP20ParseError> {
        if let Some(detail) = self.get(detail) {
            Ok(detail.clone())
        } else {
            Err(DIP20ParseError::MissingKey(detail.to_owned()))
        }
    }
}

impl Into<TxRecord> for TypedEvent<DIP20Details> {
    fn into(self) -> TxRecord {
        self.into_txrecord()
    }
}

trait MapFailed<T, E> {
    fn map_failure(self, key: &'static str) -> Result<T, E>;
}

impl<T, O> MapFailed<T, DIP20ParseError> for Result<T, O> {
    fn map_failure(self, key: &'static str) -> Result<T, DIP20ParseError> {
        self.map_err(|_| DIP20ParseError::ConversionError(key.to_owned()))
    }
}
