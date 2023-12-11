#![allow(clippy::all)]

//! Contains standards for cap.
//!
//! Current alpha standards (not fully implemented and verified):
//! - `alpha-xtc`
//! - `alpha-dip721`
//! - `alpha-dip20`
//!     - `alpha-dip20-dank`

#[cfg(feature = "alpha-xtc")]
pub mod xtc;

#[cfg(feature = "alpha-dip721")]
pub mod dip721;

#[cfg(feature = "alpha-dip20")]
pub mod dip20;
