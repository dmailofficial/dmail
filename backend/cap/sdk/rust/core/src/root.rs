//! Methods for dealing with a root bucket.
//!
//! For more information on the purpose of a root bucket, see the documentation on
//! [`RootBucket`].

use crate::Bucket;
use cap_common::transaction::IndefiniteEvent;
use cap_common::{GetBucketResponse, WithIdArg};
use ic_kit::candid::CandidType;
use ic_kit::{ic::call, Principal, RejectionCode};
use serde::{Deserialize, Serialize};

/// A root bucket.
///
/// Every token contract has a root bucket. This bucket is used for the main inserting transactions
/// into history, and organizing fetching the bucket that corresponds to a given transaction.
///
/// A root bucket implements the same interface as [`Bucket`], but with 3 additional methods.
///
/// Use [`RootBucket`]'s [`Into<Bucket>`] implementation to use a [`RootBucket`] as a [`Bucket`].
#[derive(Copy, Clone, Serialize, Deserialize, CandidType)]
pub struct RootBucket(pub Principal);

impl RootBucket {
    /// Returns a bucket that be used to query for the given transaction ID.
    pub async fn get_bucket_for(&self, id: u64) -> Result<Bucket, (RejectionCode, String)> {
        let result: (GetBucketResponse,) = call(
            self.0,
            "get_bucket_for",
            (WithIdArg { id, witness: false },),
        )
        .await?;

        Ok(Bucket(result.0.canister))
    }

    /// Inserts the given transaction and returns it's issued transaction ID.
    pub async fn insert(&self, event: &IndefiniteEvent) -> Result<u64, (RejectionCode, String)> {
        let result: (u64,) = call(self.0, "insert", (event,)).await?;

        Ok(result.0)
    }

    /// Inserts the given transactions.
    pub async fn insert_many(
        &self,
        events: &[IndefiniteEvent],
    ) -> Result<u64, (RejectionCode, String)> {
        let result: (u64,) = call(self.0, "insert_many", (events,)).await?;

        Ok(result.0)
    }

    /// The time on the canister.
    ///
    /// The time can be used to check if this bucket is on the same subnet as the caller.
    pub async fn time(&self) -> Result<u64, (RejectionCode, String)> {
        let result: (u64,) = call(self.0, "time", ()).await?;

        Ok(result.0)
    }
}
