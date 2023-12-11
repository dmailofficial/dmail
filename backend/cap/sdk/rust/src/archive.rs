use crate::{handshake, restore_pending_transactions, CapEnv, IndefiniteEvent};
use cap_sdk_core::{RootBucket, Router};
use ic_cdk::export::Principal;
use ic_kit::candid::CandidType;
use ic_kit::ic;
use ic_kit::ic::get_maybe;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, CandidType)]
pub struct Archive {
    router: Option<Principal>,
    creation_cycles: Option<u64>,
    uninitialized_root_bucket: Option<Principal>,
    root_bucket: Option<Principal>,
    local_buffer: Vec<IndefiniteEvent>,
}

/// The arguments passed to the handshake call by the user.
#[derive(Clone)]
pub(crate) struct HandshakeConfigs {
    pub router: Principal,
    pub creation_cycles: u64,
}

/// The ID of the canister that is created during the handshake.
pub(crate) struct HandshakeCreatedCanister {
    pub id: Principal,
}

/// Combines all the backup logic required for Cap in a safe way. It's recommended to use
/// this method instead of the old [CapEnv::to_archive].
///
/// # Panics
///
/// If there is an ongoing flush to the root bucket that is not finalized yet.
pub fn archive() -> Archive {
    let mut archive = Archive {
        router: None,
        creation_cycles: None,
        uninitialized_root_bucket: None,
        root_bucket: None,
        local_buffer: crate::pending_transactions(),
    };

    if let Some(config) = get_maybe::<HandshakeConfigs>() {
        archive.router = Some(config.router);
        archive.creation_cycles = Some(config.creation_cycles);
    }

    if let Some(data) = get_maybe::<HandshakeCreatedCanister>() {
        archive.uninitialized_root_bucket = Some(data.id);
    }

    if let Some(env) = get_maybe::<CapEnv>() {
        archive.router = Some(env.router.0);
        archive.root_bucket = Some(env.root.0);
    }

    archive
}

pub fn from_archive(archive: Archive) {
    restore_pending_transactions(archive.local_buffer);

    if let Some(id) = archive.uninitialized_root_bucket {
        ic::store(HandshakeCreatedCanister { id })
    }

    if let Some(root_bucket) = archive.root_bucket {
        let root = RootBucket(root_bucket);
        let router = Router(archive.router.unwrap());
        CapEnv::store(&CapEnv::create(root, router));
        return;
    }

    if let Some(creation_cycles) = archive.creation_cycles {
        handshake(creation_cycles, archive.router);
    }
}
