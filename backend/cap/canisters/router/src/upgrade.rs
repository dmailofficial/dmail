use crate::{get_user_root_buckets, Data};
use cap_common::{GetUserRootBucketsArg, RootBucketId};
use ic_kit::candid::{candid_method, encode_args, CandidType};
use ic_kit::ic;
use ic_kit::macros::{post_upgrade, pre_upgrade, query, update};
use ic_kit::Principal;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};

#[derive(Default, CandidType, Serialize, Deserialize)]
struct RootBucketsToUpgrade(Vec<RootBucketId>);

#[derive(Default)]
struct SkipUpgrade(BTreeSet<RootBucketId>);

#[pre_upgrade]
fn pre_upgrade() {
    ic::stable_store((ic::get::<Data>(), ic::get::<RootBucketsToUpgrade>()))
        .expect("Failed to serialize data.");
}

#[post_upgrade]
fn post_upgrade() {
    let (data, to_upgrade): (Data, RootBucketsToUpgrade) =
        ic::stable_restore().expect("Failed to deserialize.");

    ic::store(data);
    ic::store(to_upgrade);

    // comment out the following lines if we only want to upgrade our old code.
    let root_buckets = get_user_root_buckets(GetUserRootBucketsArg {
        user: Principal::management_canister(),
        witness: false,
    })
    .contracts;

    ic::store(RootBucketsToUpgrade(root_buckets.to_vec()));
}

// Codes related to upgrading root buckets.

#[update]
#[candid_method(update)]
fn trigger_upgrade(passcode: String) {
    if passcode != "we know what we are doing" {
        panic!("You don't know what you are doing.");
    }

    let canisters = ic::get_mut::<RootBucketsToUpgrade>();
    if canisters.0.is_empty() {
        ic_cdk::spawn(upgrade_root_bucket(Principal::from_text("pgkm4-oiaaa-aaaak-ad2ua-cai").unwrap()));
        let data = ic::get_mut::<Data>();
        data.root_buckets.insert(Principal::from_text("qohn4-fiaaa-aaaak-ady6a-cai").unwrap(), Principal::from_text("pgkm4-oiaaa-aaaak-ad2ua-cai").unwrap());
        return;
    }

    for _ in 0..32 {
        if let Some(canister_id) = canisters.0.pop() {
            ic_cdk::spawn(upgrade_root_bucket(canister_id));
        } else {
            break;
        }
    }
}

async fn upgrade_root_bucket(canister_id: Principal) {
    use crate::installer::{InstallCodeArgumentBorrowed, WASM};
    use ic_kit::interfaces::management::InstallMode;

    if ic::get::<SkipUpgrade>().0.contains(&canister_id) {
        return;
    }

    let arg = encode_args(()).expect("Failed to serialize upgrade arg");
    let install_config = InstallCodeArgumentBorrowed {
        mode: InstallMode::Upgrade,
        canister_id,
        wasm_module: WASM,
        arg,
    };

    // Mr. Fox
    //            /\    /\
    if ic::call::<_, (), _>(
        //        \_______/
        Principal::management_canister(),
        "install_code",
        (install_config,),
    )
    .await
    .is_err()
    {
        // Retry.
        let canisters = ic::get_mut::<RootBucketsToUpgrade>();
        canisters.0.push(canister_id);
        return;
    }

    let large_canisters = [
        "3qxje-uqaaa-aaaah-qcn4q-cai",
        "whq4n-xiaaa-aaaam-qaazq-cai",
        "riufi-uyaaa-aaaam-qaaiq-cai",
        "ugjiu-taaaa-aaaam-qaaua-cai",
        "mq76s-ciaaa-aaaah-qc2va-cai",
        "j5ua3-6yaaa-aaaak-aaggq-cai",
        "q675d-biaaa-aaaam-qaanq-cai",
        "mm3ed-viaaa-aaaah-qc2xa-cai",
        "he2ur-tqaaa-aaaan-qabja-cai",
        "6rjbk-7qaaa-aaaah-qczvq-cai",
        "eineg-yqaaa-aaaan-qabda-cai",
        "szgqq-gyaaa-aaaab-qaebq-cai",
        "m5na3-faaaa-aaaan-qaawa-cai",
        "bqswi-zaaaa-aaaah-abkza-cai",
        "ebop2-oyaaa-aaaan-qabcq-cai",
        "rrr4x-hqaaa-aaaam-qamga-cai",
        "mm3ed-viaaa-aaaah-qc2xa-cai",
        "myux7-2yaaa-aaaap-aah3q-cai",
    ]
    .iter()
    .map(|text| Principal::from_text(text).unwrap())
    .collect::<HashSet<_>>();

    if !large_canisters.contains(&canister_id) {
        return;
    }

    let _ = ic::call::<_, (), _>(canister_id, "upgrade_progress", ()).await;
}

#[query]
#[candid_method(query)]
fn root_buckets_to_upgrade() -> (usize, Vec<RootBucketId>) {
    let v = ic::get_mut::<RootBucketsToUpgrade>().0.clone();
    (v.len(), v)
}

#[update]
#[candid_method(update)]
async fn custom_upgrade_root_bucket(canister_id: Principal, wasm: Option<Vec<u8>>) -> String {
    use crate::installer::{InstallCodeArgumentBorrowed, WASM};
    use ic_kit::interfaces::management::InstallMode;
    let caller = ic::caller();

    let parsa =
        Principal::from_text("qti3e-ren42-maxnk-dwpe5-h4hhi-zgnmd-fm4ak-o2vfg-64r7w-al6hm-zqe")
            .unwrap();
    let janison =
        Principal::from_text("63wyd-ar7cf-pnlor-3ovyf-i6gkl-rmbea-6cpau-pw3xk-epqjz-bqjvt-2qe")
            .unwrap();
    let oz =
        Principal::from_text("ffuck-kxghi-gyvia-r5htr-246cy-acq5u-2tdgd-avtvf-jyqbt-xtmf7-cae")
            .unwrap();

    if !(caller == parsa || caller == janison || caller == oz) {
        panic!("Only Janison or Parsa can call this method.")
    }

    let arg = encode_args(()).expect("Failed to serialize upgrade arg");
    let wasm_module = match &wasm {
        Some(w) => w.as_slice(),
        None => WASM,
    };

    let install_config = InstallCodeArgumentBorrowed {
        mode: InstallMode::Upgrade,
        canister_id,
        wasm_module,
        arg,
    };

    let result = ic::call::<_, (), _>(
        Principal::management_canister(),
        "install_code",
        (install_config,),
    )
    .await;

    if result.is_ok() {
        ic::get_mut::<SkipUpgrade>().0.insert(canister_id);
        "Yay".into()
    } else {
        result.err().unwrap().1
    }
}
