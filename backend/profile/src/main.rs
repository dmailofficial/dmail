use ic_cdk::api::call::ManualReply;
use ic_cdk::api::{caller, time, trap};
use ic_cdk::export::candid::{candid_method, CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
// use num_traits::cast::ToPrimitive;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
// use std::ops::Not;

use types::*;
mod types {
    use super::*;

    #[derive(CandidType, Deserialize)]
    pub struct InitArgs {
        pub name: Option<String>,
        pub custodians: Option<HashSet<Principal>>,
    }

    #[derive(CandidType, Default, Deserialize)]
    pub struct Metadata {
        pub name: Option<String>,
        pub custodians: HashSet<Principal>,
        pub created_at: u64,
        pub upgraded_at: u64,
    }

    // for biz

    #[derive(CandidType, Debug, Deserialize)]
    pub enum ApiError {
        Unauthorized,
        Other(String),
        NotDmailAccount,
    }

    // for principal
    #[derive(CandidType, Debug, Deserialize, Clone, Hash, Eq, PartialEq)]
    pub struct Profile {
        pub avatar_url: Option<String>,
        pub desc: Option<String>,
        pub contact_list: Vec<ContactDetail>,
    }

    #[derive(Clone, CandidType, Deserialize, Debug, Hash, Eq, PartialEq)]
    pub struct ContactDetail {
        pub pid: Option<Principal>,
        // if there is no @ , then Dmail's alias else Email's address
        pub alias: String,
        pub nickname: Option<String>,
    }
}

mod ledger {
    use super::*;
    thread_local!(
        static LEDGER: RefCell<Ledger> = RefCell::new(Ledger::default());
    );

    pub fn with<T, F: FnOnce(&Ledger) -> T>(f: F) -> T {
        LEDGER.with(|ledger| f(&ledger.borrow()))
    }

    pub fn with_mut<T, F: FnOnce(&mut Ledger) -> T>(f: F) -> T {
        LEDGER.with(|ledger| f(&mut ledger.borrow_mut()))
    }

    #[derive(CandidType, Default, Deserialize)]
    pub struct Ledger {
        pub metadata: Metadata,
        pub profile_db: HashMap<Principal, Profile>,
    }

    impl Ledger {
        pub fn init_metadata(&mut self, default_custodian: Principal, args: Option<InitArgs>) {
            let metadata = self.metadata_mut();
            metadata.custodians.insert(default_custodian);
            if let Some(args) = args {
                metadata.name = args.name;
                if let Some(custodians) = args.custodians {
                    for custodians in custodians {
                        metadata.custodians.insert(custodians);
                    }
                }
            }
            metadata.created_at = time();
            metadata.upgraded_at = time();
        }

        pub fn metadata(&self) -> &Metadata {
            &self.metadata
        }

        pub fn metadata_mut(&mut self) -> &mut Metadata {
            &mut self.metadata
        }
    }
}

#[init]
#[candid_method(init)]
fn init(args: Option<InitArgs>) {
    ledger::with_mut(|ledger| ledger.init_metadata(caller(), args));
}

fn is_canister_custodian() -> Result<(), String> {
    ledger::with(|ledger| {
        ic_cdk::println!("custodian->caller().to_text()---{:?}", &caller().to_text());
        ledger
            .metadata()
            .custodians
            .contains(&caller())
            .then(|| ())
            .ok_or_else(|| "Caller is not an custodian of canister".into())
    })
}

// ==================================================================================================
// metadata
// ==================================================================================================
#[query(name = "name", manual_reply = true)]
#[candid_method(query, rename = "name")]
fn name() -> ManualReply<Option<String>> {
    ledger::with(|ledger| ManualReply::one(ledger.metadata().name.as_ref()))
}

#[query(name = "custodians", manual_reply = true)]
#[candid_method(query, rename = "custodians")]
fn custodians() -> ManualReply<HashSet<Principal>> {
    ledger::with(|ledger| ManualReply::one(&ledger.metadata().custodians))
}

#[query(name = "metadata", manual_reply = true)]
#[candid_method(query, rename = "metadata")]
fn metadata() -> ManualReply<Metadata> {
    ledger::with(|ledger| ManualReply::one(ledger.metadata()))
}

#[update(name = "setName", guard = "is_canister_custodian")]
#[candid_method(update, rename = "setName")]
fn set_name(name: String) {
    ledger::with_mut(|ledger| ledger.metadata_mut().name = Some(name));
}

#[update(name = "setCustodians", guard = "is_canister_custodian")]
#[candid_method(update, rename = "setCustodians")]
fn set_custodians(custodians: HashSet<Principal>) {
    ledger::with_mut(|ledger| ledger.metadata_mut().custodians = custodians);
}

#[query(name = "queryCaller")]
#[candid_method(query, rename = "queryCaller")]
fn query_caller() -> String {
    Principal::to_text(&ic_cdk::caller())
}

// ==================================================================================================
// service
// ==================================================================================================

#[update(name = "createUpdateAvatar", guard = "is_canister_custodian")]
#[candid_method(update, rename = "createUpdateAvatar")]
async fn create_update_avatar(pid: Principal, url: String) -> Result<(), String> {
    ledger::with_mut(|ledger| {
        if let Some(profile) = ledger.profile_db.get_mut(&pid) {
            profile.avatar_url = Some(url)
        } else {
            ledger.profile_db.insert(
                pid,
                Profile {
                    avatar_url: Some(url),
                    desc: Option::default(),
                    contact_list: Vec::new(),
                },
            );
        }
        Ok(())
    })
}

#[update(name = "createUpdateContact", guard = "is_canister_custodian")]
#[candid_method(update, rename = "createUpdateContact")]
async fn create_update_contact(pid: Principal, mut detail: ContactDetail) -> Result<(), String> {
    ledger::with_mut(|ledger| {
        match ledger.profile_db.get_mut(&pid) {
            Some(profile) => {
                if let Some(index) = profile
                    .contact_list
                    .iter()
                    .position(|x| x.pid == detail.pid)
                {
                    let old_detail = profile.contact_list.remove(index);
                    detail.nickname = old_detail.nickname
                }

                profile.contact_list.insert(0, detail);
            }
            None => {
                ledger.profile_db.insert(
                    pid,
                    Profile {
                        avatar_url: Option::default(),
                        desc: Option::default(),
                        contact_list: vec![detail],
                    },
                );
            }
        };
    });
    Ok(())
}

#[query(name = "queryAvatarByPid", guard = "is_canister_custodian")]
#[candid_method(query, rename = "queryAvatarByPid")]
fn query_avatar_by_pid(pid: Principal) -> Result<Option<String>, String> {
    ledger::with(|ledger| {
        return match ledger.profile_db.get(&pid) {
            Some(profile) => Ok(profile.avatar_url.clone()),
            None => Ok(None),
        };
    })
}

#[query(name = "queryContactByPid", guard = "is_canister_custodian")]
#[candid_method(query, rename = "queryContactByPid")]
fn query_contact_by_pid(pid: Principal) -> Result<Option<Vec<ContactDetail>>, String> {
    ledger::with_mut(|ledger| {
        return match ledger.profile_db.get(&pid) {
            Some(profile) => Ok(Some(profile.contact_list.clone())),
            None => Ok(None),
        };
    })
}

// ==================================================================================================
// upgrade
// ==================================================================================================
/// NOTE:
/// If you plan to store gigabytes of state and upgrade the code,
/// Using stable memory as the main storage is a good option to consider

#[pre_upgrade]
fn pre_upgrade() {
    ledger::with(|ledger| {
        if let Err(err) = ic_cdk::storage::stable_save::<(&ledger::Ledger,)>((ledger,)) {
            trap(&format!(
                "An error occurred when saving to stable memory (pre_upgrade): {:?}",
                err
            ));
        };
    })
}

#[post_upgrade]
fn post_upgrade() {
    ledger::with_mut(
        |ledger| match ic_cdk::storage::stable_restore::<(ledger::Ledger,)>() {
            Ok((ledger_store,)) => {
                *ledger = ledger_store;
                ledger.metadata_mut().upgraded_at = time();
            }
            Err(err) => {
                trap(&format!(
                    "An error occurred when loading from stable memory (post_upgrade): {:?}",
                    err
                ));
            }
        },
    )
}

// ==================================================================================================
// generate type
// ==================================================================================================

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    ic_cdk::export::candid::export_service!();
    std::print!("{}", __export_service());
}
