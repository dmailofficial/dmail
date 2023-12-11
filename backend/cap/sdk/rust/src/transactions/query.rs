use cap_sdk_core::Bucket;

use crate::{AsTransactionsPage, CapEnv, GetTransactionsError, GetTransactionsResponse};

/// Gets a transaction for the given page.
///
/// `page` accepts any [`Into<TransactionsPage>`].
///
/// This is implemented for [`Option<u32>`] and &[`GetTransactionsResponse`].
///
/// This allows you to query for the next page from a response, as well as
/// any given page.
///
/// # Panics
/// Panics if cap is using a multi-canister system, as it
/// is currently unsupported. In this **alpha** release.
///
/// # Examples
/// TODO
pub async fn get_transaction_page(
    page: impl AsTransactionsPage,
) -> Result<GetTransactionsResponse, GetTransactionsError> {
    let context = CapEnv::get().await;

    let bucket: Bucket = context.root.into();

    let transactions = bucket
        .get_transactions(page.page())
        .await
        .map_err(|(code, details)| GetTransactionsError::Unexpected(code, details))?;

    Ok(GetTransactionsResponse {
        transactions: transactions.data,
        next_page: transactions.page + 1,
    })
}
