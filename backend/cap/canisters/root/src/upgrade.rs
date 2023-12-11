use crate::migration::{v0, v1, v2};
use crate::Data;
use crate::{migration, InProgressReadFromStable};
use ic_cdk::{id, spawn};
use ic_kit::macros::{post_upgrade, pre_upgrade, update};
use ic_kit::{ic, Principal};
use std::collections::HashSet;

const UPGRADE_SIZE: usize = 10_000;

#[pre_upgrade]
fn pre_upgrade() {
    if ic::get_maybe::<InProgressReadFromStable>().is_some() {
        return;
    }
    // If data doesn't exits, don't rewrite the stable store.
    if let Some(data) = ic::get_maybe::<Data>() {
        ic::stable_store((data,)).expect("Failed to serialize data.");
    }
}

// Currently all of the Cap bucket's are on the following git hashes:
// 0df4c75beaf3afe00f7d360ba6c5ef955e22a3c3
// 13d2e5dcbea5eb7c42f68bb011befb07bb543eb8
// 77144ab9463fc6dab5acc0ebd099e0efeec23cf0
//
// Except these canisters:
// 3qxje-uqaaa-aaaah-qcn4q-cai  - Sonics canister
// whq4n-xiaaa-aaaam-qaazq-cai  - WICPs canister
#[post_upgrade]
pub fn post_upgrade() {
    if Principal::from_text("whq4n-xiaaa-aaaam-qaazq-cai").unwrap() == id() {
        let (data,): (Data,) = ic::stable_restore().expect("Failed to deserialize");
        ic::store(data);
        return;
    }

    let from_v0 = ["3qxje-uqaaa-aaaah-qcn4q-cai", "whq4n-xiaaa-aaaam-qaazq-cai"]
        .iter()
        .map(|text| Principal::from_text(text).unwrap())
        .collect::<HashSet<_>>();

    let more_than_10k = [
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

    let id = ic::id();

    if from_v0.contains(&id) {
        rescue().unwrap_or_else(|msg| ic::trap(&format!("Rescue failed: {}", msg)));
        return;
    }

    if more_than_10k.contains(&id) {
        let (data,): (v2::Data,) = ic::stable_restore().unwrap_or_else(|m| {
            ic::trap(&format!(
                "M10K: Could not deserialize data as v2::Data: {}",
                m
            ))
        });

        ic::store(InProgressReadFromStable::new(data));

        return;
    }

    let (data,): (Data,) = ic::stable_restore().expect("Failed to deserialize");
    ic::store(data);
}

fn rescue() -> Result<(), String> {
    let mut message = String::new();

    match migration::from_stable::<v0::Data>() {
        Ok(v0) => {
            let data = v0.migrate().migrate();
            ic::store(InProgressReadFromStable::new(data));
            return Ok(());
        }
        Err(e) => message = format!("{} - ErrV0: {}", message, e),
    }

    match migration::from_stable::<v1::Data>() {
        Ok(v1) => {
            let data = v1.migrate();
            ic::store(InProgressReadFromStable::new(data));
            return Ok(());
        }
        Err(e) => message = format!("{} - ErrV0: {}", message, e),
    }

    Err(message)
}

/// Perform the leftover tasks from the upgrade.
#[update]
pub fn upgrade_progress() {
    let number_of_spawns = {
        if ic::get_maybe::<InProgressReadFromStable>().is_none() {
            return;
        }

        let c = ic::get_mut::<InProgressReadFromStable>();
        // are we the top-level upgrade_progress call?
        let is_main = c.cursor <= 1_000;
        c.progress(UPGRADE_SIZE);

        if c.is_complete() {
            let data = c.get_data().unwrap();
            ic::store(data);
            ic::delete::<InProgressReadFromStable>();
        }

        if !is_main {
            // if we're not the top-level call, don't spawn anything.
            0
        } else {
            // ceiling division.
            (c.rem() + UPGRADE_SIZE - 1) / UPGRADE_SIZE
        }
    };

    for _ in 0..number_of_spawns {
        spawn(async {
            match ic::call::<(), (), &str>(ic::id(), "upgrade_progress", ()).await {
                Ok(_) => {}
                Err(_) => {}
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::migration::*;
    use crate::upgrade::{post_upgrade, upgrade_progress};
    use crate::{insert, insert_many, Data, Principal};
    use candid::encode_args;
    use cap_common::transaction::{DetailValue, Event, IndefiniteEvent};

    use certified_vars::Map;
    use certified_vars::{AsHashTree, Seq};
    use ic_kit::{ic, MockContext, RawHandler};

    /// Create a mock indefinite event.
    fn event(i: usize) -> IndefiniteEvent {
        IndefiniteEvent {
            caller: Principal::management_canister(),
            operation: format!("op-{}", i),
            details: vec![("something".into(), DetailValue::U64(i as u64))],
        }
    }

    fn time(i: usize) -> u64 {
        10000000000 + i as u64
    }

    fn create_events(size: usize) -> Vec<Event> {
        let mut events = Vec::with_capacity(size);

        for i in 0..size {
            events.push(event(i).to_event(time(i)));
        }

        events
    }

    fn test_rescue<F: Fn(Vec<Event>)>(id: Principal, title: &'static str, store: F) {
        MockContext::new()
            .with_handler(RawHandler::raw(Box::new(move |_, _, _, _| {
                println!("{}: Still running upgrade progress.", title);
                upgrade_progress();
                Ok(encode_args(()).unwrap())
            })))
            .with_id(id)
            .with_caller(Principal::from_text("3xwpq-ziaaa-aaaah-qcn4a-cai").unwrap())
            .inject();

        println!("{}: Creating events.", title);
        let events = create_events(25_000);

        // Write data to stable storage.
        println!("{}: Storing data to stable storage", title);
        store(events);

        // Now try to decode from v0 to latest using the post_upgrade
        println!("{}: running post_upgrade", title);
        post_upgrade();

        println!(
            "{}: sending transactions during active upgrade process",
            title
        );
        insert(event(25_000));
        insert(event(25_001));
        let _id = insert(event(25_002));
        insert_many(vec![event(25_003), event(25_004), event(25_005)]);

        // Auto called by router.
        println!("{}: initial call to upgrade_progress", title);
        upgrade_progress();

        // Now we should have data.
        let data = ic::get_maybe::<Data>().expect("Data is not created.");
        data.bucket.root_hash();
        assert_eq!(data.bucket.size(), 25_006);
    }

    #[test]
    fn test_from_v0() {
        let id = Principal::from_text("3qxje-uqaaa-aaaah-qcn4q-cai").unwrap();
        test_rescue(id, "v0", |events| {
            let data = v0::Data {
                bucket: events,
                buckets: vec![],
                next_canisters: v0::CanisterList {
                    data: vec![],
                    hash: [0; 32],
                },
                users: Default::default(),
                cap_id: Principal::from_text("lj532-6iaaa-aaaah-qcc7a-cai").unwrap(),
                contract: Principal::from_text("3xwpq-ziaaa-aaaah-qcn4a-cai").unwrap(),
                writers: Default::default(),
                allow_migration: false,
            };

            data.store();
        });
    }

    #[test]
    fn test_from_v1() {
        let id = Principal::from_text("3qxje-uqaaa-aaaah-qcn4q-cai").unwrap();
        test_rescue(id, "v1", |events| {
            let data = v1::Data {
                bucket: v1::TransactionListDe(0, ic::id(), events),
                buckets: Map::new(),
                next_canisters: Seq::new(),
                users: Default::default(),
                cap_id: Principal::from_text("lj532-6iaaa-aaaah-qcc7a-cai").unwrap(),
                contract: Principal::from_text("3xwpq-ziaaa-aaaah-qcn4a-cai").unwrap(),
                writers: Default::default(),
                allow_migration: false,
            };
            data.store();
        });
    }

    #[test]
    fn test_from_v2_10k() {
        let id = Principal::from_text("riufi-uyaaa-aaaam-qaaiq-cai").unwrap();
        test_rescue(id, "v2-10k", |events| {
            let data = v2::Data {
                bucket: v2::Bucket {
                    bucket: v1::TransactionListDe(0, ic::id(), events),
                    buckets: Map::new(),
                    next_canisters: Seq::new(),
                    contract: Principal::from_text("3xwpq-ziaaa-aaaah-qcn4a-cai").unwrap(),
                },
                users: Default::default(),
                cap_id: Principal::from_text("lj532-6iaaa-aaaah-qcc7a-cai").unwrap(),
                allow_migration: false,
                writers: Default::default(),
            };
            data.store();
        });
    }

    #[test]
    fn test_from_v2_normal() {
        let id = Principal::from_text("lhtux-ciaaa-aaaag-qakpa-cai").unwrap();
        test_rescue(id, "v2-normal", |events| {
            let data = v2::Data {
                bucket: v2::Bucket {
                    bucket: v1::TransactionListDe(0, ic::id(), events),
                    buckets: Map::new(),
                    next_canisters: Seq::new(),
                    contract: Principal::from_text("3xwpq-ziaaa-aaaah-qcn4a-cai").unwrap(),
                },
                users: Default::default(),
                cap_id: Principal::from_text("lj532-6iaaa-aaaah-qcc7a-cai").unwrap(),
                allow_migration: false,
                writers: Default::default(),
            };
            data.store();
        });
    }

    // #[test]
    // fn decode() {
    //     let data = include_bytes!("/Users/parsa/Projects/neuron-hunter/file.bin");
    //     let mut deserializer = serde_cbor::Deserializer::from_slice(data.as_slice());
    //     let value: v0::Data = serde::Deserialize::deserialize(&mut deserializer).unwrap();
    //     println!("{}", value.bucket.len());
    // }
}
