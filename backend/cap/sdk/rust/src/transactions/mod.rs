use ic_kit::RejectionCode;
use thiserror::Error;

use crate::Transaction;

mod query;
pub use query::get_transaction_page;

mod stream;
pub use stream::{get_transactions, get_user_transactions};

mod user_query;
pub use user_query::get_user_transactions_page;

/// The response given from a [`get_transactions`] call.
#[derive(Debug, Clone)]
pub struct GetTransactionsResponse {
    transactions: Vec<Transaction>,
    next_page: u32,
}

impl GetTransactionsResponse {
    /// Returns the transactions associated with this response.
    #[inline(always)]
    pub fn transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    /// Returns the next page number.
    #[inline(always)]
    pub fn next_page(&self) -> u32 {
        self.next_page
    }

    /// Converts a [`GetTransactionsResponse`] to the transactions within it.
    #[inline(always)]
    pub fn into_transactions(self) -> Vec<Transaction> {
        self.into()
    }
}

impl Into<Vec<Transaction>> for GetTransactionsResponse {
    fn into(self) -> Vec<Transaction> {
        self.transactions
    }
}

/// Convinience trait for accepting multiple types for a transaction
/// page argument.
pub trait AsTransactionsPage: Copy + Sized {
    fn page(self) -> Option<u32>;
}

impl AsTransactionsPage for Option<u32> {
    fn page(self) -> Option<u32> {
        self
    }
}

impl AsTransactionsPage for &GetTransactionsResponse {
    fn page(self) -> Option<u32> {
        Some(self.next_page)
    }
}

/// An error returned when there was an error getting transactions.
#[derive(Error, Debug)]
pub enum GetTransactionsError {
    /// The bucket rejected the call for an unexpected reason.
    #[error("the query was rejected")]
    Unexpected(RejectionCode, String),
    #[error("no transaction found with the given id")]
    InvalidId,
}
