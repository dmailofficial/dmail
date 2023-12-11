use crate::ledger::Ledger;
use crate::types::*;

use common::account_identifier::{AccountIdentifierStruct, Subaccount};
use common::principal_id::PrincipalId;

pub use ic_kit::candid::Principal;
use ic_kit::ic::trap;

 
 
use std::convert::TryInto;
use std::str::FromStr;

use ic_kit::interfaces::management::{
    CanisterSettings, CanisterStatus, CanisterStatusResponse, UpdateSettings,
    UpdateSettingsArgument, WithCanisterId,
};
use ic_kit::interfaces::Method;

pub fn caller() -> Principal {
    ic_kit::ic::caller()
}

pub fn ledger<'a>() -> &'a mut Ledger {
    ic_kit::ic::get_mut::<Ledger>()
}

pub fn token_level_metadata<'a>() -> &'a mut TokenLevelMetadata {
    ic_kit::ic::get_mut::<TokenLevelMetadata>()
}

pub fn expect_caller(input_principal: &Principal) {
    if &caller() != input_principal {
        trap("input_principal is different from caller");
    }
}

pub fn expect_caller_general(user: &User, subaccount: Option<SubAccount>) {
    match user {
        User::address(from_address) => {
            if &AccountIdentifierStruct::new(
                PrincipalId(caller()),
                Some(Subaccount(
                    subaccount
                        .expect("SubAccount is missing")
                        .try_into()
                        .expect("unable to convert SubAccount to 32 bytes array"),
                )),
            )
            .to_hex()
                != from_address
            {
                trap("input account identifier is different from caller")
            }
        }
        User::principal(principal) => expect_caller(principal),
    }
}

pub fn expect_principal(user: &User) -> Principal {
    match user {
        User::address(_) => {
            trap("only principals are allowed to preserve compatibility with Dip721")
        }
        User::principal(principal) => *principal,
    }
}

 




pub fn has_ownership(ledger: &Ledger, enquire_principal: &Principal, token_id: u64) -> bool {
    match ledger.owner_of(&token_id.to_string()) {
        Ok(owner_actual_principal) => owner_actual_principal == *enquire_principal,
        _ => false,
    }
}

pub fn has_approval(ledger: &Ledger, enquire_principal: &Principal, token_id: u64) -> bool {
    // Check if the token does exist
    if !ledger.does_token_exist(token_id) {
        return false;
    }

    // Check if owner
    // a owner has high precedence in regards of approval
    // so we return early if the enquire principal owns it
    if has_ownership(ledger, enquire_principal, token_id) {
        return true;
    }

    match ledger.get_approved(token_id) {
        Ok(user) => {
            match user {
                User::principal(principal) => principal == *enquire_principal,
                // TODO: consider the "account" case too
                _ => false,
            }
        }
        _ => false,
    }
}

pub async fn has_ownership_or_approval(
    ledger: &Ledger,
    enquire_principal: &Principal,
    to_principal: &Principal,
    token_id: u64,
) -> bool {
    // Check if the token does exist
    if !ledger.does_token_exist(token_id) {
        return false;
    }

    // On Zero address, exits immediately
    if enquire_principal.clone() == Principal::from_slice(&[0; 29]) {
        return false;
    }

    // Either has ownership, is a controller or is approved
    // otherwise, exit immediately
    if !has_ownership(ledger, enquire_principal, token_id)
        && !has_approval(ledger, enquire_principal, token_id)
        && !ledger.is_approved_for_all(enquire_principal, to_principal)
        && !is_controller(&enquire_principal).await
    {
        return false;
    }

    true
}

// TODO: check if caller is a controller
// is required that the canister has the correct controllers
// which should include the canister id itself
// to let the canister call the `aaaaa-aa` Management API `canister_status`
pub async fn is_controller(principal: &Principal) -> bool {
    let response = canister_status(ic_cdk::api::id()).await;
    ic_cdk::println!("caller:{:?}", &principal.to_text());

    match response {
        Ok(status) => {
            ic_cdk::println!(
                "controllers[0]:{:?}",
                &status.settings.controllers[0].to_text()
            );
            ic_cdk::println!(
                "controllers[1]:{:?}",
                &status.settings.controllers[1].to_text()
            );
            status.settings.controllers.contains(principal)
        }
        err => {
            ic_cdk::println!("error:------{:?}", err);
            false
        }
    }
}

async fn canister_status(canister_id: Principal) -> Result<CanisterStatusResponse, String> {
    ic_cdk::println!("current_canister_id.to_text:{:?}", canister_id.to_text());

    CanisterStatus::perform(
        Principal::management_canister(),
        (WithCanisterId { canister_id },),
    )
    .await
    .map(|(status,)| Ok(status))
    .unwrap_or_else(|(code, message)| Err(format!("Code: {:?}, Message: {}", code, message)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::*;

    use ic_kit::mock_principals::*;
    use ic_kit::MockContext;

    use ic_kit::async_test;

    fn setup_ledger(principal: &Principal) -> Ledger {
        MockContext::new().with_caller(principal.clone()).inject();

        let mut ledger = Ledger::default();
        let metadata_desc = vec![MetadataPart {
            purpose: MetadataPurpose::Rendered,
            key_val_data: vec![MetadataKeyVal {
                key: "location".to_owned(),
                val: MetadataVal::TextContent("Canister A".to_owned()),
            }],
            data: vec![],
        }];

        match ledger.mint_nft(principal, &metadata_desc) {
            Ok(_mint_receipt) => ledger,
            _ => panic!("Oops! Failed to mint nft"),
        }
    }

    #[test]
    fn test_ownership() {
        // Actors
        let alice = &alice();

        // Business logic
        let ledger = setup_ledger(alice);

        // Should Alice should have ownership
        assert_eq!(has_ownership(&ledger, alice, 0), true);
    }

    #[async_test]
    async fn test_approval() {
        // Actors
        let alice = &alice();
        let bob = &bob();
        let john = &john();

        // Business logic
        let ledger = setup_ledger(alice);

        // Before each, Alice approves Bob
        let _user = ledger.approve(bob, 0).await;

        // Should Bob be approved
        assert_eq!(has_approval(&ledger, bob, 0), true);

        // Should John have approval all
        ledger.set_approval_for_all(john, true);

        // Should John be approved in behalf of Alice
        assert_eq!(ledger.is_approved_for_all(alice, john), true);

        // Should John be approved in behalf of Alice
        assert_eq!(
            has_ownership_or_approval(&ledger, alice, john, 0).await,
            true
        );

        // Should John NOT have approval all
        ledger.set_approval_for_all(john, false);

        // Should John be NOT approved in behalf of Alice
        assert_eq!(ledger.is_approved_for_all(alice, john), false);
    }
}
