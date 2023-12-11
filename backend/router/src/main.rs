use base64;
use candid::{Encode, Nat};
use ic_cdk::api::call::ManualReply;
use ic_cdk::api::call::{call_with_payment, CallResult};
use ic_cdk::api::{caller, time, trap};
use ic_cdk::export::candid::{candid_method, CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use sha2::{Digest, Sha256};
use std::fmt::Write;
use std::str::FromStr;
// use base58::ToBase58;
// use num_traits::cast::ToPrimitive;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
// use std::ops::Not;
use types::*;
mod manager;
use manager::{
    canister_status, create_canister, deposit_cycles, install_code, update_settings, CanisterId,
    CanisterInstallArgs, CanisterSettings, CanisterStatus, CreateCanisterArgs, InstallMode,
    UpdateSettingsArg,
};
pub const WASM: &[u8] = include_bytes!("../../target/wasm32-unknown-unknown/release/body.wasm");

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
        pub router_db: HashMap<Principal, Vec<Principal>>,
        pub count1: usize,
        pub count2: usize,
    }

    #[derive(CandidType, Default, Deserialize)]
    pub struct LedgerV2 {
        pub metadata: Metadata,
        pub router_db: HashMap<Principal, Vec<Principal>>,
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

// token mechanism for interface call security
const PRIV_KEY_INIT: &str = "308184020100301006072a8648ce3d020106052b8104000a046d306b0201010420";
const PRIVATE_KEY: &str = "
-----BEGIN PRIVATE KEY-----
MIGEAgEAMBAGByqGSM49AgEGBSuBBAAKBG0wawIBAQQgaXTD7AuYMzb9WZNp3/IaQaB27dJVvFSE/uoFNoY4ebWhRANCAASAoSk0M0m0DgdPNYKi/dJIR9SLK2nDK7o8/2ZFNA04i77Y6NIQ3S10plm/LxjtusC1VQ4nSz6PJ9ArZcRcTHs1
-----END PRIVATE KEY-----";
const CALLER_SECRET_KEY: &str = "3Z8MipAm7zrBi3Wr3Ako";
#[derive(CandidType)]
struct CallerToken {
    priv_key_init: String,
    private_key: String,
}

impl CallerToken {
    fn new(priv_key_init: String, private_key: String) -> Self {
        Self {
            priv_key_init,
            private_key,
        }
    }
}
#[query(name = "getCallerToken")]
#[candid_method(query, rename = "getCallerToken")]
async fn get_private_key_info(computed_key: String) -> Result<Option<CallerToken>, String> {
    // Validate the key using a deterministic algorithm
    // 1. Perform sha256 opertation on key.
    // 2. Use the checksum algprithm(sha256),truncate the first 4 bytes.
    // 4. Do base64 formatting.

    let mut buffer = [0u8; 25];
    let once_hash = Sha256::digest(&CALLER_SECRET_KEY.as_bytes()).to_vec();

    // ic_cdk::println!("once_hash---{:?}", &once_hash);
    // ic_cdk::println!("once_hash lenght: {}", &once_hash.len());

    buffer[1..21].copy_from_slice(&once_hash[0..20]);
    let sum = &Sha256::digest(Sha256::digest(&buffer[0..21]))[0..4];
    buffer[21..25].copy_from_slice(&sum);

    let mut output_str = String::new();
    for v in buffer.iter() {
        write!(output_str, "{}", *v);
    }

    let output = base64::encode(output_str);
    // ic_cdk::println!("computed_key--{:?}", &computed_key);
    // ic_cdk::println!("output---{:?}", &output);

    if computed_key == output {
        Ok(Some(CallerToken::new(
            PRIV_KEY_INIT.to_string(),
            PRIVATE_KEY.to_string(),
        )))
    } else {
        Err("Invalid key".into())
    }
}

#[update(name = "createCidByPid", guard = "is_canister_custodian")]
#[candid_method(update, rename = "createCidByPid")]
async fn create_cid_by_pid(pid: Principal) -> Result<Principal, String> {
    match query_cid_by_pid(pid) {
        Ok(opt_cid) => match opt_cid {
            Some(cid) => Ok(cid),
            None => {
                let args = CreateCanisterArgs {
                    settings: Some(CanisterSettings {
                        controllers: Some(vec![
                            Principal::from_str(
                                "3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe",
                            )
                            .unwrap(),
                            Principal::from_str(
                                "lhlw2-d53lt-rsq5n-5iyhb-nthfo-kjvvk-lkgqx-ibgou-am7sx-g4wlt-7ae",
                            )
                            .unwrap(),
                            Principal::from_str("5zje4-caaaa-aaaak-aafqa-cai").unwrap(),
                            Principal::from_str("ej4yk-oiaaa-aaaak-aam7a-cai").unwrap(),
                        ]),
                        compute_allocation: None,
                        memory_allocation: None,
                        freezing_threshold: Some(Nat::from(2_592_000)),
                    }),
                };

                let cid = create_canister(args, 1_000_000_000_000)
                    .await
                    .unwrap()
                    .0
                    .canister_id;
                let install_args = Some(InitArgs {
                    name: Some("body".to_owned()),
                    custodians: Some(HashSet::from([
                        Principal::from_str(
                            "3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe",
                        )
                        .unwrap(),
                        Principal::from_str(
                            "lhlw2-d53lt-rsq5n-5iyhb-nthfo-kjvvk-lkgqx-ibgou-am7sx-g4wlt-7ae",
                        )
                        .unwrap(),
                    ])),
                });
                let _ = install_code(CanisterInstallArgs {
                    mode: InstallMode::Install,
                    canister_id: cid,
                    wasm_module: WASM.to_vec(),
                    arg: Encode!(&install_args).unwrap(),
                })
                .await
                .unwrap();
                ledger::with_mut(|ledger| {
                    if let Some(router) = ledger.router_db.get_mut(&pid) {
                        router.push(cid);
                    } else {
                        ledger.router_db.insert(pid, vec![cid]);
                    }
                    Ok(cid)
                })
            }
        },
        Err(err) => Err(err),
    }
}

#[update(name = "bindCid", guard = "is_canister_custodian")]
#[candid_method(update, rename = "bindCid")]
async fn bind_cid(pid: Principal, cid: Principal) -> Result<Principal, String> {
    ledger::with_mut(|ledger| {
        if let Some(router) = ledger.router_db.get_mut(&pid) {
            *router = vec![cid];
        } else {
            ledger.router_db.insert(pid, vec![cid]);
        }
        Ok(cid)
    })
}

#[update(name = "changeBindCid", guard = "is_canister_custodian")]
#[candid_method(update, rename = "changeBindCid")]
async fn change_bind_cid() -> Result<(), String> {
    ledger::with_mut(|ledger| {
        for (_, cid) in ledger.router_db.iter_mut() {
            let sub_cid = cid[0];
            if &sub_cid.to_text() == "d5ibc-yqaaa-aaaan-qc7oa-cai" {
                *cid = vec![Principal::from_str("gmvsh-myaaa-aaaak-qbphq-cai").unwrap()]
            }
        }
        Ok(())
    })
}

#[update(name = "singleUpdateSettings", guard = "is_canister_custodian")]
#[candid_method(update, rename = "singleUpdateSettings")]
async fn single_update_settings(cid: Principal) -> Result<(), String> {
    let _ = update_settings(UpdateSettingsArg {
        canister_id: cid,
        settings: manager::CanisterSettings {
            controllers: Some(vec![
                Principal::from_str(
                    "3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe",
                )
                .unwrap(),
                Principal::from_str(
                    "lhlw2-d53lt-rsq5n-5iyhb-nthfo-kjvvk-lkgqx-ibgou-am7sx-g4wlt-7ae",
                )
                .unwrap(),
                Principal::from_str("5zje4-caaaa-aaaak-aafqa-cai").unwrap(),
                Principal::from_str("ej4yk-oiaaa-aaaak-aam7a-cai").unwrap(),
            ]),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: Some(Nat::from(2_592_000)),
        },
    })
    .await;
    Ok(())
}

// not use often
#[update(name = "singleDepositCycles", guard = "is_canister_custodian")]
#[candid_method(update, rename = "singleDepositCycles")]
async fn single_deposit_cycles(cid: Principal) -> Result<(), String> {
    let _ = deposit_cycles(CanisterId { canister_id: cid }, 50000000000).await;
    Ok(())
}

#[update(name = "singleCanisterStatus", guard = "is_canister_custodian")]
#[candid_method(update, rename = "singleCanisterStatus")]
async fn single_canister_status(cid: Principal) -> Result<CanisterStatus, String> {
    let res: CanisterStatus = canister_status(CanisterId { canister_id: cid })
        .await
        .unwrap()
        .0;
    Ok(res)
}

#[update(name = "singleCanisterUpdateCode", guard = "is_canister_custodian")]
#[candid_method(update, rename = "singleCanisterUpdateCode")]
async fn single_canister_update_code(cid: Principal) -> Result<(), String> {
    let install_args = Some(InitArgs {
        name: Some("body".to_owned()),
        custodians: Some(HashSet::from([
            Principal::from_str("3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe")
                .unwrap(),
            Principal::from_str("lhlw2-d53lt-rsq5n-5iyhb-nthfo-kjvvk-lkgqx-ibgou-am7sx-g4wlt-7ae")
                .unwrap(),
        ])),
    });
    let _ = install_code(CanisterInstallArgs {
        mode: InstallMode::Upgrade,
        canister_id: cid,
        wasm_module: WASM.to_vec(),
        arg: Encode!(&install_args).unwrap(),
    })
    .await;

    Ok(())
}

#[query(name = "queryCidByPid")]
#[candid_method(query, rename = "queryCidByPid")]
fn query_cid_by_pid(pid: Principal) -> Result<Option<Principal>, String> {
    ledger::with(|ledger| {
        if let Some(router) = ledger.router_db.get(&pid) {
            Ok(Some(router[0]))
        } else {
            Ok(None)
        }
    })
}

#[query(name = "queryPidByCid", guard = "is_canister_custodian")]
#[candid_method(query, rename = "queryPidByCid")]
fn query_pid_by_cid(cid: Principal) -> Result<Option<Principal>, String> {
    ledger::with(|ledger| {
        for (pid, vec_cid) in ledger.router_db.iter() {
            for in_cid in vec_cid {
                if cid == *in_cid {
                    return Ok(Some(*pid));
                }
            }
        }
        Ok(None)
    })
}

#[query(name = "queryAllCid", guard = "is_canister_custodian")]
#[candid_method(query, rename = "queryAllCid")]
fn query_all_cid() -> Result<Vec<Vec<Principal>>, String> {
    Ok(ledger::with(|ledger| {
        ledger.router_db.values().cloned().collect::<Vec<_>>()
    }))
}

#[query]
#[candid_method(query)]
fn query_all_cid_by_split(from: usize, to: usize) -> Result<Vec<Vec<Principal>>, String> {
    Ok(ledger::with(|ledger| {
        let cid_list = ledger.router_db.values().cloned().collect::<Vec<Vec<Principal>>>();
        cid_list[from..to].to_vec()
    }))
}

#[query]
#[candid_method(query)]
fn query_all_cid_len() -> Result<usize, String> {
    Ok(ledger::with(|ledger| {
        ledger.router_db.values().cloned().collect::<Vec<_>>().len()
    }))
}

#[query]
#[candid_method(query)]
fn query_all_pid_len() -> Result<usize, String> {
    Ok(ledger::with(|ledger| {
        ledger.router_db.len()
    }))
}

// not use often
#[update(name = "queryAllUpdate", guard = "is_canister_custodian")]
#[candid_method(update, rename = "queryAllUpdate")]
async fn query_all_update() -> Result<(), String> {
    let list: Vec<Vec<Principal>> =
        ledger::with(|ledger| ledger.router_db.values().cloned().collect());
    for item in list {
        let _ = single_canister_update_code(item[0]).await;
    }
    Ok(())
}

#[query(name = "batchQueryPidByCids", guard = "is_canister_custodian")]
#[candid_method(query, rename = "batchQueryPidByCids")]
fn batch_query_pid_by_cids(cids: Vec<Principal>) -> Result<Vec<Principal>, String> {
    let mut pids = vec![];
    for cid in cids {
        if let Some(pid) = query_pid_by_cid(cid).unwrap() {
            pids.push(pid);
        } else {
            return Err(format!("Cannot find pid from cid: {}", cid.clone()));
        }
    }
    Ok(pids)
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
                // ledger.metadata = ledger_store.metadata;
                // ledger.router_db = ledger_store.router_db;
                // ledger.count1 = 0;
                // ledger.count2 = 0;
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
// helper
// ==================================================================================================

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
