use cap_common::did::*;
use cap_common::transaction::Event;
use certified_vars::{Map, Seq};
use ic_kit::candid::CandidType;
use ic_kit::candid::Principal;
use ic_kit::ic;
use ic_kit::stable::{StableReader, StableWriter};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub mod v2 {
    use super::*;

    #[derive(CandidType, Deserialize)]
    pub struct Bucket {
        pub bucket: v1::TransactionListDe,
        pub buckets: Map<TransactionId, Principal>,
        pub next_canisters: Seq<BucketId>,
        pub contract: TokenContractId,
    }

    #[derive(CandidType, Deserialize)]
    pub struct Data {
        pub bucket: Bucket,
        pub users: BTreeSet<Principal>,
        pub cap_id: Principal,
        pub allow_migration: bool,
        pub writers: BTreeSet<TokenContractId>,
    }

    impl Data {
        pub fn store(&self) {
            ic::stable_store((self,)).expect("Failed to serialize data.");
        }
    }
}

/// f18c9b48287f489ed8c4bac6f0a285b2251a7f4e
pub mod v1 {
    use super::*;

    /// Serialized transaction list.
    /// (offset, contract, events)
    #[derive(CandidType, Deserialize, Serialize)]
    pub struct TransactionListDe(pub u64, pub Principal, pub Vec<Event>);

    #[derive(Deserialize, Serialize)]
    pub struct Data {
        pub bucket: TransactionListDe,
        pub buckets: Map<TransactionId, Principal>,
        pub next_canisters: Seq<BucketId>,
        pub users: BTreeSet<Principal>,
        pub cap_id: Principal,
        pub contract: TokenContractId,
        pub writers: BTreeSet<TokenContractId>,
        pub allow_migration: bool,
    }

    impl Data {
        pub fn migrate(self) -> v2::Data {
            v2::Data {
                bucket: v2::Bucket {
                    bucket: self.bucket,
                    buckets: self.buckets,
                    next_canisters: self.next_canisters,
                    contract: self.contract,
                },
                users: self.users,
                cap_id: self.cap_id,
                allow_migration: self.allow_migration,
                writers: self.writers,
            }
        }

        pub fn store(&self) {
            let writer = StableWriter::default();
            serde_cbor::to_writer(writer, &self).expect("Failed to serialize data.");
        }
    }
}

/// 9be74b2cf8cf10cd8f9ead09eb44fb3aada01e40
pub mod v0 {
    use super::*;
    use certified_vars::Hash;

    #[derive(Deserialize, Serialize)]
    pub struct CanisterList {
        pub(crate) data: Vec<Principal>,
        pub(crate) hash: Hash,
    }

    #[derive(Deserialize, Serialize)]
    pub struct Data {
        pub bucket: Vec<Event>,
        pub buckets: Vec<(TransactionId, Principal)>,
        pub next_canisters: CanisterList,
        pub users: BTreeSet<Principal>,
        pub cap_id: Principal,
        pub contract: TokenContractId,
        pub writers: BTreeSet<TokenContractId>,
        pub allow_migration: bool,
    }

    impl Data {
        pub fn migrate(self) -> v1::Data {
            let contract = self.contract;
            let bucket = v1::TransactionListDe(0, contract, self.bucket);

            v1::Data {
                bucket,
                buckets: {
                    let mut table = Map::new();
                    table.insert(0, ic::id());
                    table
                },
                // For now we never had next_canisters,
                // so this is safe.
                next_canisters: Seq::new(),
                users: self.users,
                cap_id: self.cap_id,
                contract,
                writers: self.writers,
                allow_migration: self.allow_migration,
            }
        }

        pub fn store(&self) {
            let writer = StableWriter::default();
            serde_cbor::to_writer(writer, &self).expect("Failed to serialize data.");
        }
    }
}

pub fn from_stable<T>() -> serde_cbor::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let reader = StableReader::default();
    let mut deserializer = serde_cbor::Deserializer::from_reader(reader);
    let value = Deserialize::deserialize(&mut deserializer)?;
    // to allow TrailingData, we comment this line out.
    // deserializer.end()?;
    Ok(value)
}
