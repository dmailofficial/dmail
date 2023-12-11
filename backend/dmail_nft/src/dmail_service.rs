use std::collections::HashMap;

use derive_new::new;
use ic_kit::ic;

type Alias = String;

use ic_cdk::api::{caller, time, trap};
use ic_cdk::export::candid::{candid_method, CandidType, Deserialize, Nat};
use ic_cdk::export::Principal;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};

use crate::types::TokenDisplayList;
use crate::{
    types::{into_token_index, ApiError, TokenIdentifier, TokenIndex, TokenMetadata},
    utils::{is_controller, ledger},
};

#[update]
async fn bind(token_identifier: TokenIdentifier, pid: Principal) -> Result<bool, ApiError> {
    // let caller = ic::caller();
    // if !is_controller(&caller).await {
    //     return Err(ApiError::Unauthorized);
    // }
    let token_index: TokenIndex = into_token_index(&token_identifier);
    Ok(ledger().bind_alias_for_using_dmail(token_index, pid))
}

#[update]
async fn batch_mint_nft_to_market() -> Result<bool, ApiError> {
    let caller = ic::caller();
    if !is_controller(&caller).await {
        return Err(ApiError::Unauthorized);
    }
    Ok(ledger().batch_mint_nft_to_market().await)
}

#[query]
async fn check_mint_bind(pid: Principal, index: TokenIdentifier) -> Option<bool> {
    let caller = ic::caller();
    // if !is_controller(&caller).await {
    //     return Err(ApiError::Unauthorized);
    // }
    ledger().check_mint_bind(pid, index).await
}

#[query]
async fn display_all_token() -> Vec<TokenDisplayList> {
    ledger().all_token_display().await
}

#[update]
async fn single_mint_nft_to_market(
    mint_pid: Principal,
    index: u64,
    nft_content: String,
    bind_flag: bool,
) -> Result<bool, ApiError> {
    let caller = ic::caller();
    if !is_controller(&caller).await {
        return Err(ApiError::Unauthorized);
    }
    Ok(ledger()
        .single_mint_nft_to_market(mint_pid, index, nft_content, bind_flag)
        .await)
}

#[update]
async fn unbind(pid: Principal) -> Result<bool, ApiError> {
    let caller = ic::caller();
    if !is_controller(&caller).await {
        return Err(ApiError::Unauthorized);
    }
    Ok(ledger().unbind_alias_for_pid(pid))
}

#[update]
async fn replace_pid(token_index: u64, to: Principal) -> Result<bool, ApiError> {
    let caller = ic::caller();
    if !is_controller(&caller).await {
        return Err(ApiError::Unauthorized);
    }

    // token_index , to:new principal id
    Ok(ledger().replace_pid(token_index, to))
}

#[query]
async fn query_pid_by_alias(alias: Alias) -> Option<Principal> {
    ledger().query_pid_by_alias(alias)
}

#[query]
fn query_bind_alias_by_pid(pid: Principal) -> Option<Alias> {
    ledger().query_bind_alias_by_pid(pid)
}

#[query]
fn query_alias_by_pid(pid: Principal) -> Option<Vec<Alias>> {
    ledger().query_alias_by_pid(pid)
}

#[query(name = "queryCaller")]
fn query_caller() -> String {
    Principal::to_text(&ic_cdk::caller())
}
