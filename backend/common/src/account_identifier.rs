use crate::principal_id::*;
use crate::sha224::Sha224;

use crc32fast;
use hex;
use ic_kit::candid::types::{Serializer, Type};
use ic_kit::candid::{decode_one, encode_one, CandidType, Principal};
use serde::{de, de::Error, Deserialize, Serialize};

use std::{
    convert::{TryFrom, TryInto},
    fmt::{Display, Formatter},
    str::FromStr,
};

/// While this is backed by an array of length 28, it's canonical representation
/// is a hex string of length 64. The first 8 characters are the CRC-32 encoded
/// hash of the following 56 characters of hex. Both, upper and lower case
/// characters are valid in the input string and can even be mixed.
///
/// When it is encoded or decoded it will always be as a string to make it
/// easier to use from DFX.
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountIdentifierStruct {
    pub hash: [u8; 28],
}

pub static SUB_ACCOUNT_ZERO: Subaccount = Subaccount([0; 32]);
static ACCOUNT_DOMAIN_SEPERATOR: &[u8] = b"\x0Aaccount-id";

#[test]
fn manual_test() {
    use ic_kit::candid::Principal;
    let principal =
        Principal::from_text("a7v4m-eprbc-ecduz-h6rwf-kvcs3-vxowl-dpscn-mh3iq-lwgzg-ur4bd-jae")
            .unwrap();
    let principal_id = PrincipalId(principal);
    let account_identifier = AccountIdentifierStruct::new(principal_id, Some(Subaccount([1; 32])));
    println!("result here: {}", account_identifier.to_hex());
}

impl AccountIdentifierStruct {
    pub fn new(account: PrincipalId, sub_account: Option<Subaccount>) -> AccountIdentifierStruct {
        let mut hash = Sha224::new();
        hash.write(ACCOUNT_DOMAIN_SEPERATOR);
        hash.write(account.as_slice());

        let sub_account = sub_account.unwrap_or(SUB_ACCOUNT_ZERO);
        hash.write(&sub_account.0[..]);

        AccountIdentifierStruct {
            hash: hash.finish(),
        }
    }

    pub fn from_hex(hex_str: &str) -> Result<AccountIdentifierStruct, String> {
        let hex: Vec<u8> = hex::decode(hex_str).map_err(|e| e.to_string())?;
        Self::from_slice(&hex[..])
    }

    /// Goes from the canonical format (with checksum) encoded in bytes rather
    /// than hex to AccountIdentifier
    pub fn from_slice(v: &[u8]) -> Result<AccountIdentifierStruct, String> {
        // Trim this down when we reach rust 1.48
        let hex: Box<[u8; 32]> = match v.to_vec().into_boxed_slice().try_into() {
            Ok(h) => h,
            Err(_) => {
                let hex_str = hex::encode(v);
                return Err(format!(
                    "{} has a length of {} but we expected a length of 64",
                    hex_str,
                    hex_str.len()
                ));
            }
        };
        check_sum(*hex)
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.to_vec())
    }

    pub fn to_vec(&self) -> Vec<u8> {
        [&self.generate_checksum()[..], &self.hash[..]].concat()
    }

    pub fn generate_checksum(&self) -> [u8; 4] {
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(&self.hash);
        hasher.finalize().to_be_bytes()
    }
}

impl From<Principal> for AccountIdentifierStruct {
    fn from(principal: Principal) -> Self {
        AccountIdentifierStruct::new(PrincipalId(principal), Some(Subaccount([0; 32])))
    }
}

impl Display for AccountIdentifierStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.to_hex().fmt(f)
    }
}

impl FromStr for AccountIdentifierStruct {
    type Err = String;

    fn from_str(s: &str) -> Result<AccountIdentifierStruct, String> {
        AccountIdentifierStruct::from_hex(s)
    }
}

impl Serialize for AccountIdentifierStruct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_hex().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AccountIdentifierStruct {
    // This is the canonical way to read a this from string
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
        D::Error: de::Error,
    {
        let hex: [u8; 32] = hex::serde::deserialize(deserializer)?;
        check_sum(hex).map_err(D::Error::custom)
    }
}

impl From<PrincipalId> for AccountIdentifierStruct {
    fn from(pid: PrincipalId) -> Self {
        AccountIdentifierStruct::new(pid, None)
    }
}

fn check_sum(hex: [u8; 32]) -> Result<AccountIdentifierStruct, String> {
    // Get the checksum provided
    let found_checksum = &hex[0..4];

    // Copy the hash into a new array
    let mut hash = [0; 28];
    hash.copy_from_slice(&hex[4..32]);

    let account_id = AccountIdentifierStruct { hash };
    let expected_checksum = account_id.generate_checksum();

    // Check the generated checksum matches
    if expected_checksum == found_checksum {
        Ok(account_id)
    } else {
        Err(format!(
            "Checksum failed for {}, expected check bytes {} but found {}",
            hex::encode(&hex[..]),
            hex::encode(expected_checksum),
            hex::encode(found_checksum),
        ))
    }
}

impl CandidType for AccountIdentifierStruct {
    // The type expected for account identifier is
    fn _ty() -> Type {
        String::_ty()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        self.to_hex().idl_serialize(serializer)
    }
}

/// Subaccounts are arbitrary 32-byte values.
#[derive(Serialize, Deserialize, CandidType, Clone, Hash, Debug, PartialEq, Eq, Copy)]
#[serde(transparent)]
pub struct Subaccount(pub [u8; 32]);

impl Subaccount {
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl From<&PrincipalId> for Subaccount {
    fn from(principal_id: &PrincipalId) -> Self {
        let mut subaccount = [0; std::mem::size_of::<Subaccount>()];
        let principal_id = principal_id.as_slice();
        subaccount[0] = principal_id.len().try_into().unwrap();
        subaccount[1..1 + principal_id.len()].copy_from_slice(principal_id);
        Subaccount(subaccount)
    }
}

impl TryFrom<&Subaccount> for PrincipalId {
    type Error = PrincipalIdError;

    fn try_from(subaccount: &Subaccount) -> Result<Self, Self::Error> {
        let len = subaccount.0[0] as usize;
        let bytes = &subaccount.0[1..];
        bytes[0..len.min(bytes.len())].try_into()
    }
}

impl From<Subaccount> for Vec<u8> {
    fn from(val: Subaccount) -> Self {
        val.0.to_vec()
    }
}

impl TryFrom<&[u8]> for Subaccount {
    type Error = std::array::TryFromSliceError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        slice.try_into().map(Subaccount)
    }
}

impl Display for Subaccount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        hex::encode(self.0).fmt(f)
    }
}

#[test]
fn check_round_trip() {
    let ai = AccountIdentifierStruct { hash: [7; 28] };
    let res = ai.to_hex();
    assert_eq!(
        res.parse(),
        Ok(ai),
        "The account identifier doesn't change after going back and forth between a string"
    )
}

#[test]
fn check_encoding() {
    let ai = AccountIdentifierStruct { hash: [7; 28] };

    let en1 = encode_one(ai).unwrap();
    let en2 = encode_one(ai.to_string()).unwrap();

    assert_eq!(
        &en1, &en2,
        "Candid encoding of an account identifier and a string should be identical"
    );

    let de1: String = decode_one(&en1[..]).unwrap();
    let de2: AccountIdentifierStruct = decode_one(&en2[..]).unwrap();

    assert_eq!(
        de1.parse(),
        Ok(de2),
        "The types are the same after decoding, even through a different type"
    );

    assert_eq!(de2, ai, "And the value itself hasn't changed");
}
