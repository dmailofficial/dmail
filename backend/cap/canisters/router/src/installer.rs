use crate::Data;
use certified_vars::Seq;
use ic_kit::candid::candid_method;
use ic_kit::candid::encode_args;
use ic_kit::candid::CandidType;
use ic_kit::interfaces::management::InstallMode;
use ic_kit::interfaces::{management, Method};
use ic_kit::{ic, Principal};
use serde::Deserialize;

// It's ok.
use cap_common::*;
use ic_kit::macros::*;

pub const WASM: &[u8] =
    include_bytes!("../../../wasm/cap_root.wasm");

#[derive(CandidType, Deserialize)]
pub struct InstallCodeArgumentBorrowed<'a> {
    pub mode: InstallMode,
    pub canister_id: Principal,
    #[serde(with = "serde_bytes")]
    pub wasm_module: &'a [u8],
    pub arg: Vec<u8>,
}

#[update]
#[candid_method(update)]
async fn install_bucket_code(canister_id: RootBucketId) {
    let contract_id = ic::caller();
    install_code(canister_id, contract_id, &[]).await;
}

pub async fn install_code(canister_id: Principal, contract_id: Principal, writers: &[Principal]) {
    use management::{CanisterStatus, WithCanisterId};

    let data = ic::get_mut::<Data>();

    if data.root_buckets.get(&contract_id).is_some() {
        panic!(
            "Contract {} is already registered with a root bucket.",
            contract_id
        );
    }

    let (response,) = CanisterStatus::perform(
        Principal::management_canister(),
        (WithCanisterId { canister_id },),
    )
    .await
    .expect("Failed to retrieve canister status");

    if response.settings.controllers.len() > 1 {
        panic!("Expected one controller on canister {}", canister_id);
    }



    let arg =
        encode_args((contract_id, writers)).expect("Failed to serialize the install argument.");

    let install_config = InstallCodeArgumentBorrowed {
        mode: InstallMode::Install,
        canister_id,
        wasm_module: WASM,
        arg,
    };

    let _: () = ic::call(
        Principal::management_canister(),
        "install_code",
        (install_config,),
    )
    .await
    .expect("Install code failed.");

    data.root_buckets.insert(contract_id, canister_id);

    data.user_canisters
        .entry(Principal::management_canister())
        .or_insert(Seq::new())
        .append(canister_id);
}
