use certified_vars::Map;
use certified_vars::{
    hashtree::{fork, fork_hash},
    AsHashTree, HashTree, Seq,
};
use ic_kit::{
    candid::{candid_method, export_service, CandidType},
    ic,
    interfaces::{
        management::{CanisterStatus, CanisterStatusResponse, WithCanisterId},
        Method,
    },
    macros::*,
    Principal,
};
use serde::{Deserialize, Serialize};

// It's ok.
use cap_common::*;

mod installer;
mod plug;
mod upgrade;

/// Merkle tree of the canister.
///
/// 0: Canister Map
/// 1: User canisters
/// 2: Index canisters list
///
///      ROOT
///     /   \
///   / \    2
///  0   1
#[derive(CandidType, Serialize, Deserialize)]
pub struct Data {
    /// Map: TokenContractId -> RootBucketId
    pub root_buckets: Map<TokenContractId, RootBucketId>,
    /// Map each user to RootBucketId
    pub user_canisters: Map<UserId, Seq<RootBucketId>>,
    /// List of the index canisters.
    pub index_canisters: Seq<IndexCanisterId>,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            root_buckets: Map::new(),
            user_canisters: Map::new(),
            index_canisters: {
                let mut list = Seq::new();
                list.append(ic::id());
                list
            },
        }
    }
}

#[query]
#[candid_method(query)]
fn get_token_contract_root_bucket(
    arg: GetTokenContractRootBucketArg,
) -> GetTokenContractRootBucketResponse {
    let data = ic::get::<Data>();

    let witness = match arg.witness {
        false => None,
        true => Some(
            fork(
                fork(
                    data.root_buckets.witness(&arg.canister),
                    HashTree::Pruned(data.user_canisters.root_hash()),
                ),
                HashTree::Pruned(data.index_canisters.root_hash()),
            )
            .into(),
        ),
    };

    let canister = data.root_buckets.get(&arg.canister).cloned();

    GetTokenContractRootBucketResponse { canister, witness }
}

#[query]
#[candid_method(query)]
fn get_user_root_buckets(arg: GetUserRootBucketsArg) -> GetUserRootBucketsResponse {
    let data = ic::get::<Data>();

    let witness = match arg.witness {
        false => None,
        true => Some(
            fork(
                fork(
                    HashTree::Pruned(data.root_buckets.root_hash()),
                    data.user_canisters.witness(&arg.user),
                ),
                HashTree::Pruned(data.index_canisters.root_hash()),
            )
            .into(),
        ),
    };

    let contracts = data
        .user_canisters
        .get(&arg.user)
        .unwrap_or(&Seq::new())
        .as_vec()
        .clone();

    GetUserRootBucketsResponse { contracts, witness }
}

#[query]
#[candid_method(query)]
fn get_index_canisters(arg: WithWitnessArg) -> GetIndexCanistersResponse {
    let data = ic::get::<Data>();

    let witness = match arg.witness {
        false => None,
        true => Some(
            fork(
                HashTree::Pruned(fork_hash(
                    &data.root_buckets.root_hash(),
                    &data.user_canisters.root_hash(),
                )),
                data.index_canisters.as_hash_tree(),
            )
            .into(),
        ),
    };

    let canisters = data.index_canisters.as_vec().clone();

    GetIndexCanistersResponse { canisters, witness }
}

#[update]
#[candid_method(update)]
fn insert_new_users(contract_id: Principal, users: Vec<Principal>) {
    let data = ic::get_mut::<Data>();
    let root_bucket = ic::caller();

    assert_eq!(
        data.root_buckets.get(&contract_id),
        Some(&root_bucket),
        "Access denied."
    );

    for user in users {
        data.user_canisters
            .entry(user)
            .or_insert(Seq::new())
            .append(root_bucket);
    }
}

#[query]
#[candid_method(query)]
fn git_commit_hash() -> String {
    compile_time_run::run_command_str!("git", "rev-parse", "HEAD").into()
}

// get a root buckets status via management api
#[update]
#[candid_method(update)]
async fn bucket_status(canister_id: Principal) -> Result<CanisterStatusResponse, String> {
    CanisterStatus::perform(
        Principal::management_canister(),
        (WithCanisterId { canister_id },),
    )
    .await
    .map(|(status,)| Ok(status))
    .unwrap_or_else(|(code, message)| Err(format!("Code: {:?}, Message: {}", code, message)))
}

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
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
        write(dir.join("router.did"), export_candid()).expect("Write failed.");
    }
}
