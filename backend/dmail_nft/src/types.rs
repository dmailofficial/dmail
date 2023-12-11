use crate::ledger::Ledger;
use common::account_identifier::AccountIdentifierStruct;

use derive_new::*;
use ic_kit::candid::CandidType;
pub use ic_kit::candid::Nat;
pub use ic_kit::candid::Principal;
use serde::Deserialize;
use serde::Serialize;
use std::collections::VecDeque;

// use cap_std::dip721::DIP721TransactionType;

pub use std::convert::{From, Into};
pub use std::error::Error;
pub use std::vec::Vec;

pub type Balance = Nat;
pub type Memo = Vec<u8>;
pub type SubAccount = Vec<u8>;
pub type TokenIdentifier = String;
pub type TokenIndex = u64;
pub type AccountIdentifier = String;
pub type Date = u64;
pub type TransactionId = Nat;

pub type AccountIdentifierReturn = Result<AccountIdentifier, CommonError>;
pub type BalanceReturn = Result<Balance, CommonError>;
pub type MetadataReturn = Result<Metadata, CommonError>;
pub type Blob = Vec<u8>;

// BEGIN DIP-721 //

#[derive(CandidType, Debug, Deserialize)]
pub enum ApiError {
    Unauthorized,
    InvalidTokenId,
    ZeroAddress,
    Other,

    //dmail
    AliasFormatFail(String),
    AliasHasBeenTaken,
}

pub type TxReceipt = Result<Nat, ApiError>;

#[derive(CandidType, Deserialize)]
pub enum InterfaceId {
    Approval,
    TransactionHistory,
    Mint,
    Burn,
    TransferNotification,
}

#[derive(CandidType, Deserialize)]
pub struct LogoResult {
    logo_type: String,
    data: String,
}

pub type OwnerResult = Result<Principal, ApiError>;

#[derive(CandidType, Deserialize)]
pub struct ExtendedMetadataResult {
    pub metadata_desc: MetadataDesc,
    pub token_id: u64,
}

pub type MetadataResult = Result<MetadataDesc, ApiError>;

pub type MetadataDesc = Vec<MetadataPart>;

#[derive(CandidType, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct MetadataPart {
    pub purpose: MetadataPurpose,
    pub key_val_data: Vec<MetadataKeyVal>,
    pub data: Vec<u8>,
}

#[derive(CandidType, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum MetadataPurpose {
    Preview,
    Rendered,
}

#[derive(CandidType, Clone, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MetadataKeyVal {
    pub key: String,
    pub val: MetadataVal,
}

#[derive(CandidType, Clone, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MetadataVal {
    TextContent(String),
    BlobContent(Vec<u8>),
    NatContent(Nat),
    Nat8Content(u8),
    Nat16Content(u16),
    Nat32Content(u32),
    Nat64Content(u64),
}

// #[derive(CandidType, Deserialize)]
// pub struct TransactionResult {
//     pub fee: Nat,
//     pub transaction_type: DIP721TransactionType,
// }

#[derive(CandidType, Deserialize)]
pub struct Transfer {
    pub token_id: u64,
    pub from: Principal,
    pub to: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct TransferFrom {
    pub token_id: u64,
    pub from: Principal,
    pub to: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct Approve {
    pub token_id: u64,
    pub from: Principal,
    pub to: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct SetApprovalForAll {
    pub from: Principal,
    pub to: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct Mint {
    pub token_id: u64,
    pub to: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct Burn {
    pub token_id: u64,
}

pub type MintReceipt = Result<MintReceiptPart, ApiError>;

#[derive(CandidType, Deserialize)]
pub struct MintReceiptPart {
    pub token_id: u64,
    pub id: Nat,
}

/// END DIP-721 ///

#[allow(non_camel_case_types)]
#[derive(Clone, CandidType, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum User {
    address(AccountIdentifier),
    principal(Principal),
}

impl From<User> for AccountIdentifierStruct {
    fn from(user: User) -> Self {
        match user {
            User::principal(p) => p.into(),
            User::address(a) => {
                AccountIdentifierStruct::from_hex(&a).expect("unable to decode account identifier")
            }
        }
    }
}

impl From<User> for String {
    fn from(user: User) -> Self {
        match user {
            User::principal(p) => Into::<AccountIdentifierStruct>::into(p).to_hex(),
            User::address(a) => a,
        }
    }
}

impl From<Principal> for User {
    fn from(principal: Principal) -> Self {
        User::principal(principal)
    }
}

impl From<AccountIdentifier> for User {
    fn from(account_identifier: AccountIdentifier) -> Self {
        User::address(account_identifier)
    }
}

pub fn into_token_index(token_identifier: &TokenIdentifier) -> TokenIndex {
    token_identifier
        .parse::<u64>()
        .expect("unable to convert token identifier to token index")
}

pub fn into_token_identifier(token_index: &TokenIndex) -> TokenIdentifier {
    token_index.to_string()
}

#[derive(CandidType, Deserialize)]
pub struct TransferRequest {
    pub amount: Balance,
    pub from: User,
    pub memo: Memo,
    pub notify: bool,
    pub subaccount: Option<SubAccount>,
    pub to: User,
    pub token: TokenIdentifier,
}

#[derive(Clone, CandidType, Deserialize)]
pub enum TransferError {
    CannotNotify(AccountIdentifier),
    InsufficientBalance,
    InvalidToken(TokenIdentifier),
    Other(String),
    Rejected,
    Unauthorized(AccountIdentifier),
}

pub type TransferResponse = Result<Balance, TransferError>;

#[derive(Clone, CandidType, Deserialize)]
pub struct MintRequest {
    pub metadata: Option<MetadataContainer>,
    pub to: User,
}

#[allow(non_camel_case_types)]
#[derive(Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Metadata {
    fungible(FungibleMetadata),
    nonfungible(Option<MetadataContainer>),
}

#[derive(Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct FungibleMetadata {
    name: String,
    symbol: String,
    decimals: u8,
    metadata: Option<MetadataContainer>,
}

#[allow(non_camel_case_types)]
#[derive(Clone, CandidType, Deserialize, new, PartialEq, Eq, PartialOrd, Ord)]
pub enum MetadataContainer {
    data(Vec<MetadataValue>),
    blob(Blob),
    json(String),
}

#[derive(Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct MetadataValue(String, Value);

#[allow(non_camel_case_types)]
#[derive(Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    text(String),
    blob(Blob),
    nat(Nat),
    nat8(u8),
}

#[derive(Clone, CandidType, Debug, Deserialize)]
pub enum CommonError {
    InvalidToken(TokenIdentifier),
    Other(String),
}

#[derive(Clone, CandidType, Deserialize, new, PartialEq, Eq)]
pub struct TokenMetadata {
    pub account_identifier: AccountIdentifier,
    pub metadata: Metadata,
    pub token_identifier: TokenIdentifier,
    pub principal: Principal,
    pub metadata_desc: MetadataDesc,
}

#[derive(new, CandidType, Clone, Default, Deserialize, Serialize)]
pub struct TokenLevelMetadata {
    pub owner: Option<Principal>,
    pub symbol: String,
    pub name: String,
    pub history: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct Transaction {
    pub txid: TransactionId,
    request: TransferRequest,
    date: Date,
}

#[allow(non_camel_case_types)]
#[derive(CandidType, Deserialize)]
pub enum TransactionRequestFilter {
    txid(TransactionId),
    user(User),
    date(Date, Date),
    page(Nat, Nat),
    all,
}

#[derive(CandidType, Deserialize)]
pub struct TransactionsRequest {
    query: TransactionRequestFilter,
    token: TokenIdentifier,
}

#[derive(CandidType)]
pub struct StableStorageBorrowed<'a> {
    pub ledger: &'a Ledger,
    pub token: &'a TokenLevelMetadata,
}

#[derive(CandidType, Deserialize)]
pub struct StableStorage {
    pub ledger: Ledger,
    pub token: TokenLevelMetadata,
}

#[derive(Clone, CandidType, Deserialize, PartialEq, Eq)]
pub struct TokenDisplayList {
    pub token_id: String,
    pub principal_id: String,
    pub nft_content: String,
}
