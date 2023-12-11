//! Contains logic for interacting with cap's index canister.
//!
//! For more information on the purpose of a index canister, see the documentation on
//!['Index`].

use crate::root::RootBucket;
use crate::router::Router;
use cap_common::{
    GetIndexCanistersResponse, GetTokenContractRootBucketArg, GetTokenContractRootBucketResponse,
    GetUserRootBucketsArg, GetUserRootBucketsResponse, WithWitnessArg,
};
use ic_kit::ic::call;
use ic_kit::{Principal, RejectionCode};
use thiserror::Error;

/// A Cap index canister.
#[derive(Clone)]
pub struct Index(Principal);

impl Index {
    /// Creates a new index from the given [`Principal`]
    pub fn new(principal: Principal) -> Self {
        Self(principal)
    }

    /// Returns the root bucket canister associated with the given token contract.
    pub async fn get_token_contract_root_bucket(
        &self,
        contract: Principal,
    ) -> Result<RootBucket, GetContractRootError> {
        let result: (GetTokenContractRootBucketResponse,) = call(
            self.0,
            "get_token_contract_root_bucket",
            (GetTokenContractRootBucketArg {
                canister: contract,
                witness: false,
            },),
        )
        .await
        .map_err(|err| GetContractRootError::Rejected(err.0, err.1))?;

        if let Some(canister) = result.0.canister {
            Ok(RootBucket(canister))
        } else {
            Err(GetContractRootError::InvalidContract)
        }
    }

    /// Returns all roots for contracts a user has transactions on.
    pub async fn get_user_root_buckets(
        &self,
        user: Principal,
    ) -> Result<Vec<RootBucket>, (RejectionCode, String)> {
        let result: (GetUserRootBucketsResponse,) = call(
            self.0,
            "get_user_root_buckets",
            (GetUserRootBucketsArg {
                user,
                witness: false,
            },),
        )
        .await?;

        Ok(result
            .0
            .contracts
            .iter()
            .map(|canister| RootBucket(*canister))
            .collect())
    }

    /// Returns the list of router canisters that can be used for querying the indexes.
    pub async fn get_router_canisters(&self) -> Result<Vec<Router>, (RejectionCode, String)> {
        let result: (GetIndexCanistersResponse,) = call(
            self.0,
            "get_router_canisters",
            (WithWitnessArg { witness: false },),
        )
        .await?;

        Ok(result
            .0
            .canisters
            .iter()
            .map(|canister| Router(*canister))
            .collect())
    }
}

impl From<Router> for Index {
    fn from(router: Router) -> Self {
        Index(router.0)
    }
}

/// An error thrown when querying for a contract's root bucket.
#[derive(Error, Debug)]
pub enum GetContractRootError {
    /// The bucket rejected the call.
    #[error("the query was rejected (TODO: Display here)")]
    Rejected(RejectionCode, String),
    /// There is no root bucket for the given contract.
    #[error("no root found for the given contract")]
    InvalidContract,
}
