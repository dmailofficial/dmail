use ic_kit::ic::call;
use ic_kit::{Principal, RejectionCode};

use crate::root::RootBucket;
use cap_common::{
    GetIndexCanistersResponse, GetTransactionResponse, GetTransactionsArg, GetTransactionsResponse,
    GetUserTransactionsArg, WithIdArg, WithWitnessArg,
};

/// A contract-specific bucket canister.
///
/// A bucket canister implements storage for its parent contract. The total storage for a given
/// contract is created using multiple bucket canisters, which are interconnected using a root bucket
/// and router system. Querying buckets also features pagination.
#[derive(Copy, Clone)]
pub struct Bucket(pub(crate) Principal);

impl Bucket {
    /// Returns the list of canisters which have different pages of data.
    pub async fn get_next_canisters(&self) -> Result<Vec<Bucket>, (RejectionCode, String)> {
        let result: (GetIndexCanistersResponse,) = call(
            self.0,
            "get_next_canisters",
            (WithWitnessArg { witness: false },),
        )
        .await?;

        Ok(result
            .0
            .canisters
            .iter()
            .map(|canister| Bucket(*canister))
            .collect())
    }

    /// Returns the transaction corresponding to the passed transaction ID.
    pub async fn get_transaction(
        &self,
        id: u64,
    ) -> Result<GetTransactionResponse, (RejectionCode, String)> {
        let result: (GetTransactionResponse,) = call(
            self.0,
            "get_transaction",
            (WithIdArg { id, witness: false },),
        )
        .await?;

        Ok(result.0)
    }

    /// Returns all of the transactions for this contract.
    pub async fn get_transactions(
        &self,
        page: Option<u32>,
    ) -> Result<GetTransactionsResponse, (RejectionCode, String)> {
        let result: (GetTransactionsResponse,) = call(
            self.0,
            "get_transactions",
            (GetTransactionsArg {
                page,
                witness: false,
            },),
        )
        .await?;

        Ok(result.0)
    }

    /// Returns all of the transactions associated with the given user.
    pub async fn get_user_transactions(
        &self,
        user: Principal,
        page: Option<u32>,
    ) -> Result<GetTransactionsResponse, (RejectionCode, String)> {
        let result: (GetTransactionsResponse,) = call(
            self.0,
            "get_user_transactions",
            (GetUserTransactionsArg {
                user,
                page,
                witness: false,
            },),
        )
        .await?;

        Ok(result.0)
    }
}

impl From<RootBucket> for Bucket {
    fn from(root: RootBucket) -> Self {
        Bucket(root.0)
    }
}
