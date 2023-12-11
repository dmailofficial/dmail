use candid::Encode;
use ic_cdk::api::call::ManualReply;
use ic_cdk::api::{caller, time, trap};
use ic_cdk::export::candid::{candid_method, CandidType, Deserialize, Nat};
use ic_cdk::export::Principal;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use std::str::FromStr;
// use num_traits::cast::ToPrimitive;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
// use std::ops::Not;
use types::*;

mod types {
    use super::*;

    #[derive(CandidType, Deserialize, Debug)]
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

    // #[derive(CandidType, Debug, Deserialize, Clone)]
    // pub struct EmailBO {
    //     pub email_header: EmailHeader,
    //     pub email_body: EmailBody,
    // }

    #[derive(CandidType, Debug, Deserialize, Clone)]
    pub struct EmailAgent {
        pub cid :Principal,
        pub email_header: EmailHeader,
        pub email_body: EmailBody,
    }

    #[derive(CandidType, Debug, Deserialize, Clone)]
    pub struct EmailHeader {
        pub id: u64,
        pub parent_id: Option<u64>,
        pub subject: String,
        pub s_pid: Option<Principal>,
        pub r_pid: Option<Principal>,
        pub s_alias: String,
        pub r_alias: String,
        pub created_at: u64,
    }

    #[derive(CandidType, Debug, Deserialize, Clone)]
    pub struct EmailBody {
        pub content: String,
        pub assets:String,
        pub attachment_list: Vec<Attachment>,
    }

    #[derive(CandidType, Debug, Deserialize, Clone)]
    pub enum SendStatus {
        Sending,
        Success,
        Fail,
        Draft,
    }

    #[derive(CandidType, Debug, Deserialize, Clone)]
    pub struct Attachment {
        pub id: u64,
        pub name: String,
        pub size: Nat,
        pub mime_type: String,
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
        pub web2_email_cache_db: Vec<EmailAgent>,
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
// #[query(name = "name", manual_reply = true)]
// #[candid_method(query, rename = "name")]
// fn name() -> ManualReply<Option<String>> {
//     ledger::with(|ledger| ManualReply::one(ledger.metadata().name.as_ref()))
// }

#[query(name = "custodians", manual_reply = true)]
#[candid_method(query, rename = "custodians")]
fn custodians() -> ManualReply<HashSet<Principal>> {
    ledger::with(|ledger| ManualReply::one(&ledger.metadata().custodians))
}

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

#[update(name = "setCustodians", guard = "is_canister_custodian")]
#[candid_method(update, rename = "setCustodians")]
fn set_custodians(custodians: HashSet<Principal>) {
    ledger::with_mut(|ledger| ledger.metadata_mut().custodians = custodians);
}

// #[query(name = "queryCaller")]
// #[candid_method(query, rename = "queryCaller")]
// fn query_caller() -> String {
//     Principal::to_text(&ic_cdk::caller())
// }

// ==================================================================================================
// service
// ==================================================================================================

#[update(name = "queryWeb2MailCache", guard = "is_canister_custodian")]
#[candid_method(update, rename = "queryWeb2MailCache")]
async fn query_web2_mail_cache(first_n: usize) -> Result<Vec<EmailAgent>, String> {
    ledger::with_mut(|ledger| {
        let len = ledger.web2_email_cache_db.len().min(first_n);
        let new_list = ledger.web2_email_cache_db.drain(0..len).collect();
        ic_cdk::println!("query_web2_mail_cache: \n{:?}", &new_list);
        Ok(new_list)
    })
}

// #[update(name = "updateWeb2MailCache", guard = "is_canister_custodian")]
// #[candid_method(update, rename = "updateWeb2MailCache")]
// async fn update_web2_email_cache(first_n: usize) -> Result<(), String> {
//     let update_list = ledger::with_mut(|ledger| {
//         let len = ledger.web2_email_cache_db.len().min(first_n);
//         return ledger.web2_email_cache_db.drain(0..len).collect::<Vec<_>>();
//     });

//     ic_cdk::println!("web2 update header status successfully");
//     Ok(())
// }

#[update(name = "createAnWeb2Mail")]
#[candid_method(update, rename = "createAnWeb2Mail")]
fn create_an_web2_mail(mut email_bo: EmailAgent) -> Result<(), String> {
    if email_bo.email_header.r_alias.contains('@') && email_bo.email_header.r_pid.is_none() {
        ledger::with_mut(|ledger| {
            email_bo.email_header.created_at = ic_cdk::api::time();
            ledger.web2_email_cache_db.push(email_bo);
            Ok(())
        })
    } else {
        Err("Not a Web2 Email address.".to_owned())
    }
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
// foreign
// ==================================================================================================

// async fn update_web3_mail_status(list: Vec<(Principal, EmailBO)>) {
//     let canister_id = Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap();
//     for i in list {
//         let _: Result<(Result<(), String>,), _> =
//             ic_cdk::api::call::call(i.0, "updateWeb3MailStatus", (i.1,)).await;
//     }
// }

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
