use crate::dmail_util::get_points_by_len;
use crate::dmail_util::normalize_str;
use crate::dmail_util::validate_alias;
use crate::foreign_call::mint_nft_to_market;
use crate::foreign_call::transfer_from_nft_market;
use crate::types::*;
use crate::utils::*;

use ic_kit::ic;
use ic_kit::ic::trap;
use ic_kit::macros::*;
use std::str::FromStr;
// use ic_cdk::export::Principal;

/// HEALTH-CHECK ///
#[query]
fn name() -> String {
    String::from("NFT Canister")
}

#[query(name = "balanceOfDip721")]
fn balance_of_dip721(user: Principal) -> u64 {
    ledger().balance_of(&user.into())
}

#[query(name = "ownerOfDip721")]
fn owner_of_dip721(token_id: u64) -> Result<Principal, ApiError> {
    ledger().owner_of(&token_id.to_string())
}

#[update(name = "safeTransferFromDip721")]
async fn safe_transfer_from_dip721(from: Principal, to: Principal, token_id: u64) -> TxReceipt {
    let ledger_instance = ledger();
    let caller = ic::caller();

    // We're interesting in knowing if the `caller` has permissions
    // but also, verify that it is approved to work in behalf of `from`
    // otherwise not authorised
    if !has_ownership_or_approval(ledger_instance, &caller, &to, token_id).await
        || !has_ownership_or_approval(ledger_instance, &from, &to, token_id).await
    {
        return Err(ApiError::Unauthorized);
    }

    assert_ne!(
        to,
        Principal::from_slice(&[0; 29]),
        "transfer request to cannot be the zero principal"
    );

    ledger().transfer(
        &User::principal(from),
        &User::principal(to),
        &token_id.to_string(),
    );

    Ok(Nat::from(1))
}

#[update(name = "transferFromDip721")]
async fn transfer_from_dip721(from: Principal, to: Principal, token_id: u64) -> TxReceipt {
    match get_metadata_dip721(token_id) {
        Ok(result) => {
            if result[0].key_val_data[1].val == MetadataVal::TextContent("true".to_string()) {
                return Err(ApiError::Other);
            }
        }
        Err(err) => return Err(err),
    }

    let ledger_instance = ledger();
    let caller = ic::caller();

    // We're interesting in knowing if the `caller` has permissions
    // but also, verify that it is approved to work in behalf of `from`
    // otherwise not authorised
    if !has_ownership_or_approval(ledger_instance, &caller, &to, token_id).await
        || !has_ownership_or_approval(ledger_instance, &from, &to, token_id).await
    {
        return Err(ApiError::Unauthorized);
    }

    ledger().transfer(
        &User::principal(from),
        &User::principal(to),
        &token_id.to_string(),
    );

    // Dmail
    let _ = transfer_from_nft_market(to, from, "Dmail".to_owned(), token_id)
        .await
        .unwrap();

    Ok(Nat::from(1))
}

#[query(name = "supportedInterfacesDip721")]
fn supported_interfaces_dip721() -> Vec<InterfaceId> {
    vec![InterfaceId::Mint, InterfaceId::TransactionHistory]
}

#[query(name = "logoDip721")]
fn logo_dip721() -> LogoResult {
    unimplemented!();
}

#[query(name = "nameDip721")]
fn name_dip721() -> &'static str {
    &token_level_metadata().name
}

#[query(name = "symbolDip721")]
fn symbol_dip721() -> &'static str {
    &token_level_metadata().symbol
}

#[query(name = "totalSupplyDip721")]
fn total_supply_dip721() -> u64 {
    ledger().total_supply()
}

#[query(name = "getMetadataDip721")]
fn get_metadata_dip721(token_id: u64) -> MetadataResult {
    ledger().get_metadata(token_id)
}

#[query(name = "getMaxLimitDip721")]
fn get_max_limit_dip721() -> u16 {
    200
}

#[allow(unreachable_code, unused_variables)]
#[query(name = "getMetadataForUserDip721")]
fn get_metadata_for_user_dip721(user: Principal) -> Vec<ExtendedMetadataResult> {
    ledger().get_metadata_for_user(&user)
}

#[allow(unreachable_code, unused_variables)]
#[query(name = "getTokenIdsForUserDip721")]
fn get_token_ids_for_user_dip721(user: Principal) -> Vec<u64> {
    ledger().get_token_ids_for_user(&user)
}

// Implementations are encouraged to only allow minting by the owner of the smart contract
#[update(name = "mintDip721")]
async fn mint_dip721(to: Principal, metadata_desc: MetadataDesc) -> MintReceipt {
    let caller = ic::caller();
    if !is_controller(&caller).await {
        return Err(ApiError::Unauthorized);
    }

    let mut metadata_desc = metadata_desc;
    if let MetadataVal::TextContent(alias) = metadata_desc[0].key_val_data[0].val.clone() {
        validate_alias(&alias)?;
        let normalize_alias = normalize_str(alias);
        metadata_desc[0].key_val_data[0].val = MetadataVal::TextContent(normalize_alias.clone());
        ic_cdk::println!("alias----:{:?}", &metadata_desc[0].key_val_data[0].val);

        let response = ledger().mint_nft(&to, &metadata_desc)?;

        let _ = mint_nft_to_market(
            to,
            response.token_id,
            format!(
                r#"{{"alias":"{}","points":{}}}"#,
                normalize_alias,
                get_points_by_len(normalize_alias.len())
            ),
            false,
        )
        .await;

        Ok(MintReceiptPart {
            token_id: response.token_id,
            id: Nat::from(0),
        })
    } else {
        Err(ApiError::AliasFormatFail("Valid alias".to_owned()))
    }
}

#[update]
async fn transfer(transfer_request: TransferRequest) -> TransferResponse {
    let token_id = &transfer_request.token.parse::<u64>().unwrap();
    let ledger_instance = ledger();
    let caller = ic::caller();
    let to_principal = match &transfer_request.to {
        User::principal(principal) => principal,
        // TODO: Should take into consideration the user address
        _ => panic!("Oops! Unexpected transfer request to"),
    };

    if !has_ownership_or_approval(ledger_instance, &caller, &to_principal, *token_id).await {
        return Err(TransferError::Unauthorized("Unauthorized".to_string()));
    }

    expect_principal(&transfer_request.from);
    expect_principal(&transfer_request.to);
    assert_ne!(
        transfer_request.from, transfer_request.to,
        "transfer request from and to cannot be the same"
    );

    expect_caller_general(&transfer_request.from, transfer_request.subaccount);

    ledger_instance.transfer(
        &User::principal(caller),
        &transfer_request.to,
        &transfer_request.token,
    );

    Ok(Nat::from(1))
}

#[update(name = "approveDip721")]
async fn approve_dip721(spender: Principal, token_id: u64) -> Result<User, ApiError> {
    let enquire_principal = &ic::caller();
    let ledger_instance = ledger();

    if !has_ownership_or_approval(&ledger_instance, &enquire_principal, &spender, token_id).await {
        return Err(ApiError::Unauthorized);
    }

    ledger_instance.approve(&spender, token_id).await
}

#[query]
fn bearer(token_identifier: TokenIdentifier) -> AccountIdentifierReturn {
    ledger().bearer(&token_identifier)
}

#[allow(unreachable_code, unused_variables)]
#[query(name = "getAllMetadataForUser")]
fn get_all_metadata_for_user(user: User) -> Vec<TokenMetadata> {
    trap("Disabled as current EXT metadata doesn't allow multiple assets per token");
    ledger().get_all_metadata_for_user(&user)
}

#[query]
fn supply(token_identifier: TokenIdentifier) -> BalanceReturn {
    ledger().supply(&token_identifier)
}

#[allow(unreachable_code, unused_variables)]
#[query]
fn metadata(token_identifier: TokenIdentifier) -> MetadataReturn {
    trap("Disabled as current EXT metadata doesn't allow multiple assets per token");
    ledger().metadata(&token_identifier)
}

fn store_data_in_stable_store() {
    let data = StableStorageBorrowed {
        ledger: ledger(),
        token: token_level_metadata(),
    };
    ic::stable_store((data,)).expect("failed");
}

fn restore_data_from_stable_store() {
    let (data,): (StableStorage,) = ic::stable_restore().expect("failed");
    ic::store(data.ledger);
    ic::store(data.token);
}

#[init]
fn init(owner: Principal, symbol: String, name: String, history: Principal) {
    *token_level_metadata() = TokenLevelMetadata::new(Some(owner), symbol, name, Some(history));
}

#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::api::print(format!("Executing preupgrade"));
    store_data_in_stable_store();
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::api::print(format!("Executing postupgrade"));
    restore_data_from_stable_store();
}
