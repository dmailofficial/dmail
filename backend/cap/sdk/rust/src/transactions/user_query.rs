use cap_sdk_core::Bucket;
use ic_kit::Principal;

use crate::{AsTransactionsPage, CapEnv, GetTransactionsError, GetTransactionsResponse};

/// Gets the transaction with the given id.
///
/// # Panics
/// Panics if cap is using a multi-canister system, as it
/// is currently unsupported. In this **alpha** release.
///
/// # Examples
/// TODO
pub async fn get_user_transactions_page(
    user: Principal,
    page: impl AsTransactionsPage,
) -> Result<GetTransactionsResponse, GetTransactionsError> {
    let context = CapEnv::get().await;

    let as_bucket: Bucket = context.root.into();

    let transactions = as_bucket
        .get_user_transactions(user, page.page())
        .await
        .map_err(|(code, details)| GetTransactionsError::Unexpected(code, details))?;

    Ok(GetTransactionsResponse {
        transactions: transactions.data,
        next_page: transactions.page + 1,
    })
}
