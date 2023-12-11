use crate::did::EventHash;
use certified_vars::HashTree::Pruned;
use certified_vars::{AsHashTree, Hash, HashTree};
use ic_kit::candid::{CandidType, Deserialize, Nat};
use ic_kit::Principal;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::convert::TryInto;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Event {
    /// The timestamp in ms.
    pub time: u64,
    /// The caller that initiated the call on the token contract.
    pub caller: Principal,
    /// The operation that took place.
    pub operation: String,
    /// Details of the transaction.
    pub details: Vec<(String, DetailValue)>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct IndefiniteEvent {
    /// The caller that initiated the call on the token contract.
    pub caller: Principal,
    /// The operation that took place.
    pub operation: String,
    /// Details of the transaction.
    pub details: Vec<(String, DetailValue)>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DetailValue {
    True,
    False,
    U64(u64),
    I64(i64),
    Float(f64),
    Text(String),
    Principal(Principal),
    #[serde(with = "serde_bytes")]
    Slice(Vec<u8>),
    Vec(Vec<DetailValue>),
    TokenIdU64(u64),
}

impl Event {
    /// Return a set containing all of the Principal IDs involved in an event.
    #[inline]
    pub fn extract_principal_ids(&self) -> BTreeSet<&Principal> {
        let mut principals = BTreeSet::new();

        principals.insert(&self.caller);

        fn visit<'a>(principals: &mut BTreeSet<&'a Principal>, value: &'a DetailValue) {
            match value {
                DetailValue::Principal(p) => {
                    principals.insert(p);
                }
                DetailValue::Vec(items) => {
                    for item in items {
                        visit(principals, item);
                    }
                }
                _ => {}
            }
        }

        for (_, value) in &self.details {
            visit(&mut principals, value);
        }

        principals
    }

    /// Return a set containing all of the token ids involved in an event.
    #[inline]
    pub fn extract_token_ids(&self) -> BTreeSet<u64> {
        let mut tokens = BTreeSet::new();

        fn visit(tokens: &mut BTreeSet<u64>, value: &DetailValue) {
            match value {
                DetailValue::TokenIdU64(id) => {
                    tokens.insert(*id);
                }
                DetailValue::Vec(items) => {
                    for item in items {
                        visit(tokens, item);
                    }
                }
                _ => {}
            }
        }

        for (_, value) in &self.details {
            visit(&mut tokens, value);
        }

        tokens
    }

    /// Compute the hash for the given event.
    pub fn hash(&self) -> EventHash {
        let mut h = domain_sep(&self.operation);

        h.update(&self.time.to_be_bytes() as &[u8]);
        let caller = self.caller.as_slice();
        h.update(&caller.len().to_be_bytes() as &[u8]);
        h.update(caller);

        fn hash_value(h: &mut Sha256, value: &DetailValue) {
            match value {
                DetailValue::True => {
                    h.update(&[0]);
                }
                DetailValue::False => {
                    h.update(&[1]);
                }
                DetailValue::U64(val) => {
                    let bytes = val.to_be_bytes();
                    h.update(&[2]);
                    h.update(&bytes.len().to_be_bytes() as &[u8]);
                    h.update(bytes);
                }
                DetailValue::I64(val) => {
                    let bytes = val.to_be_bytes();
                    h.update(&[3]);
                    h.update(&bytes.len().to_be_bytes() as &[u8]);
                    h.update(bytes);
                }
                DetailValue::Float(val) => {
                    let bytes = val.to_be_bytes();
                    h.update(&[4]);
                    h.update(&bytes.len().to_be_bytes() as &[u8]);
                    h.update(bytes);
                }
                DetailValue::Text(val) => {
                    let bytes = val.as_str().as_bytes();
                    h.update(&[5]);
                    h.update(&bytes.len().to_be_bytes() as &[u8]);
                    h.update(bytes);
                }
                DetailValue::Principal(val) => {
                    let bytes = val.as_slice();
                    h.update(&[6]);
                    h.update(&bytes.len().to_be_bytes() as &[u8]);
                    h.update(bytes);
                }
                DetailValue::Slice(val) => {
                    let bytes = val.as_slice();
                    h.update(&[7]);
                    h.update(&bytes.len().to_be_bytes() as &[u8]);
                    h.update(bytes);
                }
                DetailValue::Vec(val) => {
                    h.update(&[8]);
                    h.update(&val.len().to_be_bytes() as &[u8]);
                    for item in val.iter() {
                        hash_value(h, item);
                    }
                }
                DetailValue::TokenIdU64(val) => {
                    let bytes = val.to_be_bytes();
                    h.update(&[9]);
                    h.update(&bytes.len().to_be_bytes() as &[u8]);
                    h.update(bytes);
                }
            }
        }

        for (key, value) in &self.details {
            h.update(&key.len().to_be_bytes() as &[u8]);
            h.update(key.as_str().as_bytes());
            hash_value(&mut h, value);
        }

        h.finalize().into()
    }
}

impl Into<IndefiniteEvent> for Event {
    fn into(self) -> IndefiniteEvent {
        IndefiniteEvent {
            caller: self.caller,
            operation: self.operation,
            details: self.details,
        }
    }
}

impl IndefiniteEvent {
    /// Convert an indefinite event to a definite one by adding the token and time fields.
    #[inline]
    pub fn to_event(self, time: u64) -> Event {
        Event {
            time,
            caller: self.caller,
            operation: self.operation,
            details: self.details,
        }
    }
}

impl From<u64> for DetailValue {
    fn from(num: u64) -> Self {
        Self::U64(num)
    }
}

impl TryInto<u64> for DetailValue {
    type Error = ();

    fn try_into(self) -> Result<u64, Self::Error> {
        if let Self::U64(num) = self {
            Ok(num)
        } else {
            Err(())
        }
    }
}

impl From<i64> for DetailValue {
    fn from(num: i64) -> Self {
        Self::I64(num)
    }
}

impl TryInto<i64> for DetailValue {
    type Error = ();

    fn try_into(self) -> Result<i64, Self::Error> {
        if let Self::I64(num) = self {
            Ok(num)
        } else {
            Err(())
        }
    }
}

impl From<f64> for DetailValue {
    fn from(float: f64) -> Self {
        Self::Float(float)
    }
}

impl TryInto<f64> for DetailValue {
    type Error = ();

    fn try_into(self) -> Result<f64, Self::Error> {
        if let Self::Float(num) = self {
            Ok(num)
        } else {
            Err(())
        }
    }
}

impl From<String> for DetailValue {
    fn from(string: String) -> Self {
        Self::Text(string)
    }
}

impl TryInto<String> for DetailValue {
    type Error = ();

    fn try_into(self) -> Result<String, Self::Error> {
        if let Self::Text(val) = self {
            Ok(val)
        } else {
            Err(())
        }
    }
}

impl From<Principal> for DetailValue {
    fn from(principal: Principal) -> Self {
        Self::Principal(principal)
    }
}

impl TryInto<Principal> for DetailValue {
    type Error = ();

    fn try_into(self) -> Result<Principal, Self::Error> {
        if let Self::Principal(principal) = self {
            Ok(principal)
        } else {
            Err(())
        }
    }
}

impl From<Nat> for DetailValue {
    fn from(nat: Nat) -> Self {
        let mut vec = vec![];

        nat.encode(&mut vec).unwrap();

        DetailValue::Slice(vec)
    }
}

impl TryInto<Nat> for DetailValue {
    type Error = ();

    fn try_into(self) -> Result<Nat, Self::Error> {
        if let Self::Slice(nat) = self {
            if let Ok(nat) = Nat::parse(&nat) {
                Ok(nat)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

fn domain_sep(s: &str) -> sha2::Sha256 {
    let buf: [u8; 1] = [s.len() as u8];
    let mut h = sha2::Sha256::new();
    h.update(&buf[..]);
    h.update(s.as_bytes());
    h
}

impl AsHashTree for Event {
    fn root_hash(&self) -> Hash {
        self.hash()
    }

    fn as_hash_tree(&self) -> HashTree<'_> {
        Pruned(self.hash())
    }
}

// TODO(qti3e) Test
