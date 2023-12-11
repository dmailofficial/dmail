#![allow(clippy::from_over_into)]
#![allow(clippy::result_unit_err)]
#![allow(dead_code)]

//! The SDK for integrating Certified Asset Provenance (CAP) into an Internet Computer canister.
//!
//! # ⚠️ **Currently the SDK is in version `1.0.0-alpha1`** ⚠️
//!
//! It does not support the full Cap specification, which may
//! cause panics if your canister is not updated when Cap is fully released.
//!
//! All methods which may suffer from issues are documented.
//!
//! # Additional Developer Resources
//! There are developer resources such as diagrams, additional
//! documentation, and examples in the [cap-sdk respository](https://github.com/Psychedelic/cap/tree/cap-sdk/sdk).
//!
//! # ⚠️ Updating your canister ⚠️
//!
//! ⚠️ There is a specific flow you need to follow when upgrading
//! your canister that depends on Cap to ensure no Cap configuration
//! is lost accidentally.
//!
//! TODO, write example

pub use env::*;
mod env;

mod transactions;
pub use transactions::*;

mod transaction;
pub use transaction::*;

mod details;
pub use details::*;

mod event;
pub use event::*;

mod archive;
pub use archive::*;

mod handshake;
pub use handshake::*;

pub use cap_sdk_core::transaction::{DetailValue, Event, IndefiniteEvent};

type Transaction = Event;
type TransactionId = u64;

#[doc(hidden)]
pub mod prelude {
    pub use crate::*;
}
