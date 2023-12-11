// foreign call

use ic_cdk_macros::update;
use ic_kit::Principal;
use std::str::FromStr;

pub async fn transfer_from_nft_market(
    new_pid: Principal,
    old_pid: Principal,
    nft_theme: String,
    index: u64,
) -> Result<bool, String> {
    let call_result: Result<(Result<bool, String>,), _> = ic_cdk::api::call::call(
        Principal::from_str("wyj5p-yqaaa-aaaak-acj2q-cai").unwrap(),
        "transfer",
        (new_pid, old_pid, nft_theme, index),
    )
    .await;
    call_result.unwrap().0
}


#[update]
pub async fn mint_nft_to_market(
    mint_pid: Principal,
    index: u64,
    nft_content: String,
    bind_flag: bool,
) -> Result<(), String> {
    let call_result: Result<(Result<(), String>,), _> = ic_cdk::api::call::call(
        Principal::from_str("wyj5p-yqaaa-aaaak-acj2q-cai").unwrap(),
        "mint",
        ("Dmail".to_owned(), mint_pid, index, nft_content, bind_flag),
    )
    .await;
    call_result.unwrap().0
}
