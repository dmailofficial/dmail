use cap_common::transaction::{Event, IndefiniteEvent};
use certified_vars::AsHashTree;
use ic_kit::candid::{candid_method, export_service, CandidType};
use ic_kit::{ic, Principal};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use crate::multi_stage_reader::InProgressReadFromStable;
use cap_common::bucket::Bucket;
use cap_common::did::*;
use ic_kit::macros::*;

mod migration;
mod multi_stage_reader;
pub mod upgrade;

/// Merkle tree of the canister.
///
/// 0: Bucket
/// 1: Buckets Lookup Map
/// 2: Next buckets
///
///      ROOT
///     /   \
///   / \    2
///  0   1
#[derive(CandidType, Serialize, Deserialize)]
pub struct Data {
    pub bucket: Bucket,
    pub users: BTreeSet<Principal>,
    pub cap_id: Principal,
    pub allow_migration: bool,
    pub writers: BTreeSet<TokenContractId>,
}

struct OldData(Data);

impl Default for OldData {
    fn default() -> Self {
        let mut data = Data::default();
        data.bucket.bucket.global_offset = 0;
        Self(data)
    }
}

impl Default for Data {
    fn default() -> Self {
        if Principal::from_text("whq4n-xiaaa-aaaam-qaazq-cai").unwrap() == ic::id() {
            let writers = vec![Principal::from_text("v6dvh-qawes-jzboy-lai6t-bzqrc-f5hym-clbrc-tdmht-x3msr-uzqgi-4qe").unwrap()];
            return Self {
                bucket: Bucket::new(Principal::from_text("utozz-siaaa-aaaam-qaaxq-cai").unwrap(), 276092),
                users: BTreeSet::new(),
                cap_id: Principal::from_text("lj532-6iaaa-aaaah-qcc7a-cai").unwrap(),
                allow_migration: false,
                writers: writers.into_iter().collect(),
            };
        }

        Self {
            bucket: Bucket::new(Principal::management_canister(), 0),
            users: BTreeSet::new(),
            cap_id: Principal::management_canister(),
            allow_migration: true,
            writers: BTreeSet::new(),
        }
    }
}

#[init]
fn init(contract: Principal, writers: BTreeSet<Principal>) {
    let data = ic::get_mut::<Data>();
    data.cap_id = ic::caller();
    data.bucket = Bucket::new(contract, 0);
    data.writers = writers;
}

#[query]
#[candid_method(query)]
fn get_upgrade_status() -> (usize, bool) {
    ic::get_maybe::<InProgressReadFromStable>()
        .expect("Not running an upgrade")
        .status()
}

#[query]
#[candid_method(query)]
fn get_stable(offset: usize, size: usize) -> Vec<u8> {
    let mut buf = vec![0; size];
    ic::stable_read(offset as u32, buf.as_mut_slice());
    buf
}

#[query]
#[candid_method(query)]
fn get_stable_size() -> u32 {
    ic::stable_size()
}

#[query]
#[candid_method(query)]
fn get_next_canisters(arg: WithWitnessArg) -> GetNextCanistersResponse {
    ic::get::<Data>().bucket.get_next_canisters(arg)
}

#[query]
#[candid_method(query)]
fn get_transaction(arg: WithIdArg) -> GetTransactionResponse {
    ic::get::<Data>().bucket.get_transaction(arg)
}

#[query]
#[candid_method(query)]
fn get_transactions(arg: GetTransactionsArg) -> GetTransactionsResponseBorrowed<'static> {
    ic::get::<Data>().bucket.get_transactions(arg)
}

#[query]
#[candid_method(query)]
fn get_user_transactions(arg: GetUserTransactionsArg) -> GetTransactionsResponseBorrowed<'static> {
    ic::get::<Data>().bucket.get_user_transactions(arg)
}

#[query]
#[candid_method(query)]
fn get_token_transactions(
    arg: GetTokenTransactionsArg,
) -> GetTransactionsResponseBorrowed<'static> {
    ic::get::<Data>().bucket.get_token_transactions(arg)
}

#[query]
#[candid_method(query)]
fn get_bucket_for(arg: WithIdArg) -> GetBucketResponse {
    ic::get::<Data>().bucket.get_bucket_for(arg)
}

#[query]
#[candid_method(query)]
fn time() -> u64 {
    ic::time()
}

#[query]
#[candid_method(query)]
fn size() -> u64 {
    ic::get::<Data>().bucket.size()
}

#[query]
#[candid_method(query)]
fn contract_id() -> &'static Principal {
    ic::get::<Data>().bucket.contract_id()
}

#[update]
#[candid_method(update)]
fn insert(event: IndefiniteEvent) -> TransactionId {
    if ic::get_maybe::<InProgressReadFromStable>().is_some() {
        let caller = ic::caller();
        return ic::get_mut::<InProgressReadFromStable>().insert_batch(&caller, vec![event]);
    }

    let data = ic::get_mut::<Data>();
    let caller = ic::caller();



    let event = event.to_event(ic::time() / 1_000_000);

    let mut new_users = Vec::new();
    for principal in event.extract_principal_ids() {
        if data.users.insert(*principal) {
            new_users.push(*principal);
        }
    }

    #[cfg(not(test))]
    ic_cdk::spawn(write_new_users_to_cap(
        data.cap_id,
        *data.bucket.contract_id(),
        new_users,
    ));

    let id = data.bucket.insert(event);

    data.allow_migration = false;

    ic::set_certified_data(&data.bucket.root_hash());

    id
}

#[update]
#[candid_method(update)]
fn insert_many(transactions: Vec<IndefiniteEvent>) -> TransactionId {
    if ic::get_maybe::<InProgressReadFromStable>().is_some() {
        let caller = ic::caller();
        return ic::get_mut::<InProgressReadFromStable>().insert_batch(&caller, transactions);
    }

    let data = ic::get_mut::<Data>();
    let caller = ic::caller();
    let time = ic::time() / 1_000_000;



    let id = data.bucket.size();
    let mut new_users = Vec::new();

    for tx in transactions {
        let event = tx.to_event(time);

        for principal in event.extract_principal_ids() {
            if data.users.insert(*principal) {
                new_users.push(*principal);
            }
        }

        data.bucket.insert(event);
    }

    ic_cdk::spawn(write_new_users_to_cap(
        data.cap_id,
        *data.bucket.contract_id(),
        new_users,
    ));

    ic::set_certified_data(&data.bucket.root_hash());

    id
}

#[update]
#[candid_method(update)]
fn migrate(events: Vec<Event>) {
    if ic::get_maybe::<InProgressReadFromStable>().is_some() {
        ic::trap("Migration is not allowed during a read from stable.");
    }
    let data = ic::get_mut::<Data>();
    let caller = ic::caller();



    if !data.allow_migration {
        ic::trap("Migration is not allowed after an insert.")
    }

    let mut new_users = Vec::new();

    for event in events {
        for principal in event.extract_principal_ids() {
            if data.users.insert(*principal) {
                new_users.push(*principal);
            }
        }

        data.bucket.insert(event);
    }

    #[cfg(not(test))]
    ic_cdk::spawn(write_new_users_to_cap(
        data.cap_id,
        *data.bucket.contract_id(),
        new_users,
    ));

    ic::set_certified_data(&data.bucket.root_hash());
}

pub async fn write_new_users_to_cap(
    cap_id: Principal,
    contract_id: Principal,
    users: Vec<Principal>,
) {
    for _ in 0..10 {
        let args = (contract_id, &users);
        if ic::call::<(Principal, &Vec<Principal>), (), &str>(cap_id, "insert_new_users", args)
            .await
            .is_ok()
        {
            break;
        }
    }
}

#[query]
#[candid_method(query)]
fn balance() -> u64 {
    ic::balance()
}

#[query]
#[candid_method(query)]
fn git_commit_hash() -> String {
    compile_time_run::run_command_str!("git", "rev-parse", "HEAD").into()
}

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[update]
fn write_data_restore(events: Vec<Event>) {
    let tmp = ic::get_mut::<OldData>();
    let data = &mut tmp.0;
    let caller = ic::caller();



    for event in events {
        data.bucket.insert(event);
    }
}

#[update]
fn complete_data_restore() {
    let mut data = ic::take::<OldData>().unwrap().0;
    let caller = ic::caller();



    let new_data = ic::take::<Data>().unwrap_or_default();

    for event in new_data.bucket.bucket.events.iter() {
        let event = unsafe { event.as_ref().clone() };
        data.bucket.insert(event);
    }

    ic::store::<Data>(data);
}

#[query]
fn old_data_size() -> u64 {
    let tmp = ic::get::<OldData>();
    tmp.0.bucket.size()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let dir = dir.parent().unwrap().parent().unwrap().join("candid");
        write(dir.join("root.did"), export_candid()).expect("Write failed.");
    }
}
