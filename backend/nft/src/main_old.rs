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

    #[derive(CandidType, Debug, Deserialize, Clone)]
    pub struct NFT {
        pub index: u64,
        pub cid: Principal,
        pub nft_theme: String,
        pub nft_content: String,
        pub bind: bool,
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

    // #[derive(CandidType, Default, Deserialize)]
    // pub struct LedgerV2 {
    //     pub metadata: Metadata,
    //     // HashMap<user,<nft_theme,<index,NFT>>> base on user
    //     pub nft_db: HashMap<Principal, HashMap<String, HashMap<u64, NFT>>>,
    //     // HashMap<nft_store_cid,nft_theme>
    //     // for register NFT market
    //     pub nft_theme_db: HashMap<Principal, String>,
    // }

    #[derive(CandidType, Default, Deserialize)]
    pub struct Ledger {
        pub metadata: Metadata,
        // HashMap<user,<nft_theme,<index,NFT>>> base on user
        pub nft_db: HashMap<Principal, HashMap<String, HashMap<u64, NFT>>>,
        // HashMap<nft_store_cid,nft_theme>
        // for register NFT market
        pub nft_theme_db: HashMap<Principal, String>,
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
// #[query(name = "name", manual_reply = true)]
// #[candid_method(query, rename = "name")]
// fn name() -> ManualReply<Option<String>> {
//     ledger::with(|ledger| ManualReply::one(ledger.metadata().name.as_ref()))
// }

// #[query(name = "custodians", manual_reply = true)]
// #[candid_method(query, rename = "custodians")]
// fn custodians() -> ManualReply<HashSet<Principal>> {
//     ledger::with(|ledger| ManualReply::one(&ledger.metadata().custodians))
// }

// #[query(name = "metadata", manual_reply = true)]
// #[candid_method(query, rename = "metadata")]
// fn metadata() -> ManualReply<Metadata> {
//     ledger::with(|ledger| ManualReply::one(ledger.metadata()))
// }

// #[update(name = "setName", guard = "is_canister_custodian")]
// #[candid_method(update, rename = "setName")]
// fn set_name(name: String) {
//     ledger::with_mut(|ledger| ledger.metadata_mut().name = Some(name));
// }

// #[update(name = "setCustodians", guard = "is_canister_custodian")]
// #[candid_method(update, rename = "setCustodians")]
// fn set_custodians(custodians: HashSet<Principal>) {
//     ledger::with_mut(|ledger| ledger.metadata_mut().custodians = custodians);
// }

// #[query(name = "queryCaller")]
// #[candid_method(query, rename = "queryCaller")]
// fn query_caller() -> String {
//     Principal::to_text(&ic_cdk::caller())
// }

// ==================================================================================================
// service
// ==================================================================================================

#[update(name = "register", guard = "is_canister_custodian")]
#[candid_method(update, rename = "register")]
fn register(nft_theme: String, register_cid: Principal) -> bool {
    ledger::with_mut(|ledger| ledger.metadata_mut().custodians.insert(register_cid));
    ledger::with_mut(|ledger| ledger.nft_theme_db.insert(register_cid, nft_theme));
    return true;
}

#[update(name = "mint")]
#[candid_method(update, rename = "mint")]
fn mint(
    nft_theme: String,
    mint_pid: Principal,
    index: u64,
    nft_content: String,
) -> Result<(), String> {
    ledger::with_mut(|ledger| {
        let cid = caller();
        ic_cdk::println!("cid:{}", cid.to_text());
        let valid_nft_theme = ledger.nft_theme_db.get(&cid).unwrap();
        if valid_nft_theme == &nft_theme {
            if let Some(nft_theme_list) = ledger.nft_db.get_mut(&mint_pid) {
                if let Some(nft_list) = nft_theme_list.get_mut(&nft_theme) {
                    nft_list.insert(
                        index,
                        NFT {
                            index,
                            cid,
                            bind: false,
                            nft_theme,
                            nft_content,
                        },
                    );
                    Ok(())
                } else {
                    nft_theme_list.insert(
                        nft_theme.clone(),
                        HashMap::from([(
                            index,
                            NFT {
                                index,
                                cid,
                                bind: false,
                                nft_theme,
                                nft_content,
                            },
                        )]),
                    );
                    Ok(())
                }
            } else {
                ledger.nft_db.insert(
                    mint_pid,
                    HashMap::from([(
                        nft_theme.clone(),
                        HashMap::from([(
                            index,
                            NFT {
                                index,
                                cid,
                                bind: false,
                                nft_theme,
                                nft_content,
                            },
                        )]),
                    )]),
                );
                Ok(())
            }
        } else {
            return Err("CALLER INVALID.".to_owned());
        }
    })
}

#[update(name = "bind", guard = "is_canister_custodian")]
#[candid_method(update, rename = "bind")]
fn bind(pid: Principal, index: u64) -> Result<bool, String> {
    ledger::with_mut(|ledger| {
        if let Some(nft_theme_list) = ledger.nft_db.get_mut(&pid) {
            if let Some(nft_list) = nft_theme_list.get_mut("Dmail") {
                if let Some(nft) = nft_list.get_mut(&index) {
                    nft.bind = true;
                    Ok(true)
                } else {
                    Err("INDEX INVALID.".to_owned())
                }
            } else {
                Err("THEME INVALID.".to_owned())
            }
        } else {
            Err("PID INVALID.".to_owned())
        }
    })
}

#[update(name = "query", guard = "is_canister_custodian")]
#[candid_method(update, rename = "query")]
fn query(pid: Principal) -> Result<Vec<NFT>, String> {
    ledger::with(|ledger| {
        if let Some(nft_theme_list) = ledger.nft_db.get(&pid) {
            let list = nft_theme_list
                .values()
                .cloned()
                .flat_map(|inner_hash_map| inner_hash_map.into_values())
                .collect::<Vec<NFT>>();
            Ok(list)
        } else {
            Err("PID INVALID.".to_owned())
        }
    })
}

#[update(name = "transfers", guard = "is_canister_custodian")]
#[candid_method(update, rename = "transfers")]
fn transfers(
    new_pid: Principal,
    old_pid: Principal,
    nft_theme: String,
    index: u64,
) -> Result<bool, String> {
    ledger::with_mut(|ledger| {
        let cid = caller();
        let valid_nft_theme = ledger.nft_theme_db.get(&cid).unwrap();
        if valid_nft_theme == &nft_theme {
            if let Some(nft_theme_list) = ledger.nft_db.get_mut(&old_pid) {
                if let Some(nft_list) = nft_theme_list.get_mut(&nft_theme) {
                    if let Some(nft) = nft_list.get(&index) {
                        let nft_clone = nft.clone();
                        let _ = mint(nft_clone.nft_theme, new_pid, index, nft_clone.nft_content);
                        let _ = nft_list.remove(&index);
                        Ok(true)
                    } else {
                        Err("INDEX INVALID.".to_owned())
                    }
                } else {
                    Err("THEME INVALID.".to_owned())
                }
            } else {
                Err("PID INVALID.".to_owned())
            }
        } else {
            Err("CALLER INVALID.".to_owned())
        }
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
        // |ledger| match ic_cdk::storage::stable_restore::<(ledger::LedgerV2,)>() {
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
