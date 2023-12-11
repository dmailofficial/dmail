//! `cap_core` is a low-level library for interacting with [cap](https://github.com/Psychedelic/cap/) from an IC
//! canister.
//!
//! If you're looking to interact with cap, you may be looking for the `cap_dk` instead.

mod bucket;

pub use bucket::Bucket;

mod index;

pub use index::{GetContractRootError, Index};

mod root;

pub use root::RootBucket;

mod router;

pub use router::Router;

pub use cap_common::*;
