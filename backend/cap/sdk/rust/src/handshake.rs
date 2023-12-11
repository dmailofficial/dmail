use std::str::FromStr;

use cap_sdk_core::{Index, Router};
use ic_kit::{
    ic,
    interfaces::{management, Method},
    Principal,
};

use crate::{CapEnv, HandshakeConfigs, HandshakeCreatedCanister};

/// Handshakes with Cap. This either grabs important metadata or
/// creates the root bucket for this contract and gives it `creation_cycles`
/// cycles.
///
/// # Panics
///
/// If there is less then 1TC left on the canister after we create the root bucket, this
/// method panics.
///
/// # Creation cycles
///
/// On main net, we require you to at least provide 5TC for your root bucket. So please do that.
pub fn handshake(creation_cycles: u64, router_override: Option<Principal>) {
    let router_pid = router_override
        .unwrap_or_else(|| Principal::from_str("lj532-6iaaa-aaaah-qcc7a-cai").unwrap());
    let router = Router::new(router_pid);

    let create_settings = management::CanisterSettings {
        controllers: Some(vec![router_pid]),
        compute_allocation: None,
        memory_allocation: None,
        freezing_threshold: None,
    };

    let arg = management::CreateCanisterArgument {
        settings: Some(create_settings),
    };

    // The canister is not already created, so let's check the cycle balance.
    if ic::get_maybe::<HandshakeCreatedCanister>().is_none() {
        let balance = ic::balance();
        let expected_balance = creation_cycles + 1_000_000_000_000;

        if balance < expected_balance {
            ic::trap("Not enough cycles on the canister to create a root bucket.")
        }
    }

    ic::store(HandshakeConfigs {
        router: router_pid,
        creation_cycles,
    });

    let closure = async move {
        let index: Index = router.into();

        if let Ok(bucket) = index.get_token_contract_root_bucket(ic::id()).await {
            CapEnv::store(&CapEnv::create(bucket, router));
            return;
        }

        // In case the canister was already created and backed up.
        let canister_id = if let Some(backup) = ic::get_maybe::<HandshakeCreatedCanister>() {
            backup.id
        } else if let Ok((res,)) = management::CreateCanister::perform_with_payment(
            Principal::management_canister(),
            (arg,),
            creation_cycles,
        )
        .await
        {
            res.canister_id
        } else {
            // Try to handshake again.
            handshake(creation_cycles, router_override);
            return;
        };

        ic::store(HandshakeCreatedCanister { id: canister_id });

        router.install_code(canister_id).await.unwrap();

        if let Ok(root_bucket) = index.get_token_contract_root_bucket(ic::id()).await {
            CapEnv::store(&CapEnv::create(root_bucket, router));
            return;
        }

        // Try to handshake again.
        handshake(creation_cycles, router_override);
    };

    CapEnv::insert_future(Box::pin(closure))
}
