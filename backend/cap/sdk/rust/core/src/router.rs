//! Contains logic for interacting with cap's main router.
//!
//! For more information on the purpose of the main router, see the documentation on
//! [`Router`].

use ic_kit::candid::CandidType;
use ic_kit::{ic::call, Principal, RejectionCode};
use serde::{Deserialize, Serialize};

/// A router.
///
/// The main router is an extended version of [`Index`] which allows registration of a new
/// contract to cap. It also contains global index canister registry which allows querying
/// for the root bucket of a contract.
///
/// A router bucket implements the same interface as [`Index`], but with 1 additional method.
///
/// Use [`Router`]'s [`Into<Index>`] implementation to use a [`Router`] as an [`Index`].
#[derive(Debug, Clone, Copy, Serialize, Deserialize, CandidType)]
pub struct Router(pub Principal);

impl Router {
    pub fn new(principal: Principal) -> Self {
        Self(principal)
    }

    pub async fn install_code(&self, canister: Principal) -> Result<(), (RejectionCode, String)> {
        call(self.0, "install_bucket_code", (canister,)).await?;

        Ok(())
    }
}
