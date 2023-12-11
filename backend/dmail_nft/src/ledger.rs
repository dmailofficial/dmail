use crate::dmail_util::get_points_by_len;
use crate::foreign_call::mint_nft_to_market;
use crate::types::*;
use crate::utils::*;

use ic_kit::{candid::CandidType, ic};

use serde::Deserialize;
use std::collections::HashMap;
use std::convert::Into;
use std::default::Default;

type Approvals = Vec<User>;

#[derive(CandidType, Clone, Default, Deserialize)]
pub struct Ledger {
    tokens: HashMap<TokenIndex, TokenMetadata>,
    user_tokens: HashMap<User, Vec<TokenIndex>>,
    token_approvals: HashMap<TokenIndex, User>,
    operator_approvals: HashMap<User, Approvals>,
}

//for Dmail
impl Ledger {
    pub fn query_pid_by_alias(&self, alias: String) -> Option<Principal> {
        for token_metadata_item in ledger().tokens.values() {
            if MetadataVal::TextContent(alias.clone())
                == token_metadata_item.metadata_desc[0].key_val_data[0].val
            {
                return Some(token_metadata_item.principal);
            }
        }
        None
    }

    pub async fn all_token_display(&self) -> Vec<TokenDisplayList> {
        let mut list = ledger()
            .tokens
            .values()
            .map(|all_nft_detail| TokenDisplayList {
                token_id: all_nft_detail.token_identifier.clone(),
                principal_id: all_nft_detail.principal.to_text(),
                nft_content: match all_nft_detail.metadata_desc[0].key_val_data[0].val.clone() {
                    MetadataVal::TextContent(alias) => alias,
                    _ => todo!(),
                },
            })
            .collect::<Vec<TokenDisplayList>>();
        list.sort_by(|a, b| {
            a.token_id
                .parse::<u64>()
                .unwrap()
                .cmp(&b.token_id.parse::<u64>().unwrap())
        });
        list
    }

    pub fn bind_alias_for_using_dmail(&mut self, token_index: TokenIndex, pid: Principal) -> bool {
        let token_metadata = ledger()
            .tokens
            .get_mut(&token_index)
            .expect("unable to find token index");

        if token_metadata.principal == pid {
            for token_metadata_item in ledger().tokens.values_mut() {
                if token_metadata_item.principal == pid {
                    token_metadata_item.metadata_desc[0].key_val_data[1].val =
                        MetadataVal::TextContent("false".to_string());
                }
            }
            token_metadata.metadata_desc[0].key_val_data[1].val =
                MetadataVal::TextContent("true".to_string());
            return true;
        }
        false
    }

    pub async fn batch_mint_nft_to_market(&mut self) -> bool {
        for (index, item) in ledger().tokens.iter() {
            let mint_pid = item.principal;
            let nft_content = match item.metadata_desc[0].key_val_data[0].val.clone() {
                MetadataVal::TextContent(alias) => {
                    format!(
                        r#"{{"alias":"{}","points":{}}}"#,
                        alias,
                        get_points_by_len(alias.len())
                    )
                }
                _ => todo!(),
            };
            let bind_flag = item.metadata_desc[0].key_val_data[1].val
                == MetadataVal::TextContent("true".to_owned());
            let _ = mint_nft_to_market(mint_pid, *index, nft_content, bind_flag).await;
        }
        true
    }

    pub async fn check_mint_bind(
        &mut self,
        pid: Principal,
        index1: TokenIdentifier,
    ) -> Option<bool> {
        for (index, item) in ledger().tokens.iter() {
            if pid == item.principal && &into_token_index(&index1) == index {
                return Some(
                    item.metadata_desc[0].key_val_data[1].val
                        == MetadataVal::TextContent("true".to_owned()),
                );
            }
            // let mint_pid = item.principal;
            // let nft_content = match item.metadata_desc[0].key_val_data[0].val.clone() {
            //     MetadataVal::TextContent(alias) => {
            //         format!(
            //             r#"{{"alias":"{}","points":{}}}"#,
            //             alias,
            //             get_points_by_len(alias.len())
            //         )
            //     }
            //     _ => todo!(),
            // };
            // let bind_flag = match item.metadata_desc[0].key_val_data[1].val.clone() {
            //     MetadataVal::TextContent(bind_status) => bind_status == "true",
            //     _ => todo!(),
            // };
            // let _ = mint_nft_to_market(mint_pid, *index, nft_content, bind_flag).await;
        }
        None
    }

    pub async fn single_mint_nft_to_market(
        &mut self,
        mint_pid: Principal,
        index: u64,
        nft_content: String,
        bind_flag: bool,
    ) -> bool {
        let nft_content = format!(
            r#"{{"alias":"{}","points":{}}}"#,
            nft_content,
            get_points_by_len(nft_content.len())
        );
        let _ = mint_nft_to_market(mint_pid, index, nft_content, bind_flag).await;
        true
    }

    pub fn unbind_alias_for_pid(&mut self, pid: Principal) -> bool {
        for token_metadata in ledger().tokens.values_mut() {
            let token_bind_status = token_metadata.metadata_desc[0].key_val_data[1].val.clone();
            if token_metadata.principal == pid
                && token_bind_status == MetadataVal::TextContent("true".to_string())
            {
                token_metadata.metadata_desc[0].key_val_data[1].val =
                    MetadataVal::TextContent("false".to_string());
                return true;
            }
        }
        false
    }

    pub fn query_bind_alias_by_pid(&mut self, pid: Principal) -> Option<String> {
        for token_metadata_item in ledger().tokens.values() {
            if token_metadata_item.principal == pid
                && token_metadata_item.metadata_desc[0].key_val_data[1].val
                    == MetadataVal::TextContent("true".to_string())
            {
                match token_metadata_item.metadata_desc[0].key_val_data[0]
                    .val
                    .clone()
                {
                    MetadataVal::TextContent(alias) => return Some(alias),
                    _ => return None,
                }
            }
        }
        None
    }

    pub fn query_alias_by_pid(&self, pid: Principal) -> Option<Vec<String>> {
        let mut alias_list = Vec::new();

        for token_metadata_item in ledger().tokens.values() {
            if token_metadata_item.principal == pid {
                match token_metadata_item.metadata_desc[0].key_val_data[0]
                    .val
                    .clone()
                {
                    MetadataVal::TextContent(alias) => alias_list.push(alias),
                    _ => continue,
                }
            }
        }

        Some(alias_list)
    }

    pub fn replace_pid(&mut self, token_index: TokenIndex, to: Principal) -> bool {
        if let Some(token_list) = ledger().tokens.get_mut(&token_index) {
            // old principal remove token_index
            if let Some(list) = ledger()
                .user_tokens
                .get_mut(&User::principal(token_list.principal))
            {
                *list = list
                    .iter()
                    .filter(|&&x| x != token_index)
                    .cloned()
                    .collect::<Vec<_>>();
            }

            // update principal
            ledger().tokens.insert(
                token_index,
                TokenMetadata::new(
                    User::principal(to.clone()).into(),
                    Metadata::nonfungible(None),
                    into_token_identifier(&token_index),
                    to.clone(),
                    token_list.metadata_desc.clone(),
                ),
            );
            ic_cdk::println!(
                "1111:{:?}",
                &ledger()
                    .tokens
                    .get(&token_index)
                    .unwrap()
                    .principal
                    .to_text()
            );

            // new principal add token_index
            ledger()
                .user_tokens
                .entry(User::principal(to))
                .or_default()
                .push(token_index);

            let ggg = ledger().user_tokens.get_mut(&User::principal(to)).unwrap();
            ic_cdk::println!("new principal:{:?}", &ggg);
            true
        } else {
            false
        }
    }
}

impl Ledger {
    pub fn mint_nft(&mut self, to: &Principal, metadata_desc: &MetadataDesc) -> MintReceipt {
        // AliasHasBeenTaken
        for token_metadata_item in ledger().tokens.values() {
            if metadata_desc[0].key_val_data[0].val
                == token_metadata_item
                    .metadata_desc
                    .get(0)
                    .unwrap()
                    .key_val_data[0]
                    .val
            {
                return Err(ApiError::AliasHasBeenTaken);
            }
        }

        let token_index = 9500 + ledger().tokens.len() as TokenIndex;
        ledger().tokens.insert(
            token_index,
            TokenMetadata::new(
                User::principal(to.clone()).into(),
                Metadata::nonfungible(None),
                into_token_identifier(&token_index),
                to.clone(),
                metadata_desc.clone(),
            ),
        );
        ledger()
            .user_tokens
            .entry(User::principal(*to))
            .or_default()
            .push(token_index);

        Ok(MintReceiptPart {
            token_id: token_index as u64,
            id: Nat::from(1),
        })
    }

    pub fn total_supply(&self) -> u64 {
        ledger().tokens.len() as u64
    }

    pub fn get_metadata(&self, token_id: u64) -> MetadataResult {
        MetadataResult::Ok(
            ledger()
                .tokens
                .get(&into_token_index(&token_id.to_string()))
                .expect("unable to find token index")
                .metadata_desc
                .clone(),
        )
    }

    pub fn get_metadata_for_user(&self, user: &Principal) -> Vec<ExtendedMetadataResult> {
        ledger()
            .user_tokens
            .get(&User::principal(*user))
            .unwrap_or(&vec![])
            .iter()
            .map(|token_index| {
                let user_tokens = ledger()
                    .tokens
                    .get(token_index)
                    .expect("unable to find token index");
                ExtendedMetadataResult {
                    metadata_desc: user_tokens.metadata_desc.clone(),
                    token_id: *token_index as u64,
                }
            })
            .collect()
    }

    pub fn get_token_ids_for_user(&self, user: &Principal) -> Vec<u64> {
        ledger()
            .user_tokens
            .get(&User::principal(*user))
            .unwrap_or(&vec![])
            .iter()
            .map(|token_index| token_index.clone() as u64)
            .collect()
    }

    pub async fn approve(
        &self,
        approves_principal: &Principal,
        token_id: u64,
    ) -> Result<User, ApiError> {
        ledger()
            .token_approvals
            .insert(token_id, User::from(approves_principal.clone()))
            .ok_or(ApiError::Other)
    }

    pub fn set_approval_for_all(&self, approves_principal: &Principal, approved: bool) {
        let user = User::principal(ic::caller());

        if ic::caller() == approves_principal.clone() {
            return;
        }

        let approvals = ledger().operator_approvals.entry(user.clone()).or_default();

        if !approved {
            if let Some(index) = approvals
                .iter()
                .position(|listed_user| *listed_user == User::principal(*approves_principal))
            {
                approvals.remove(index);
            }

            return;
        }

        approvals.push(User::from(approves_principal.clone()));
    }

    pub fn is_approved_for_all(&self, owner: &Principal, operator: &Principal) -> bool {
        let approvals = ledger()
            .operator_approvals
            .get(&User::principal(owner.clone()));

        approvals.map_or(false, |list| {
            list.contains(&User::principal(operator.clone()))
        })
    }

    pub fn get_approved(&self, token_id: u64) -> Result<User, ApiError> {
        let approved_result = ledger().token_approvals.get(&token_id);

        match approved_result {
            Some(user) => Ok(user.clone()),
            None => Err(ApiError::Unauthorized),
        }
    }

    // TODO: Seems best to return the first controller in the list
    // as the owner, as such the owner field should be removed from the contructor
    pub fn owner_of(&self, token_identifier: &TokenIdentifier) -> OwnerResult {
        let token_result = ledger().tokens.get(&into_token_index(&token_identifier));

        match token_result {
            Some(token_metadata) => OwnerResult::Ok(token_metadata.principal.clone()),
            _ => OwnerResult::Err(ApiError::InvalidTokenId),
        }
    }

    pub fn balance_of(&self, user: &User) -> u64 {
        ledger().user_tokens.get(user).unwrap_or(&vec![]).len() as u64
    }

    pub fn transfer(&mut self, from: &User, to: &User, token_identifier: &TokenIdentifier) {
        // change token owner in the tokens map
        let token_index = into_token_index(token_identifier);
        let mut token_metadata = ledger()
            .tokens
            .get_mut(&token_index)
            .expect("unable to find token identifier in tokens");

        token_metadata.account_identifier = to.clone().into();
        token_metadata.principal = expect_principal(&to);

        // remove the token from the previous owner's tokenlist
        let from_token_indexes = ledger()
            .user_tokens
            .get_mut(&from)
            .expect("unable to find previous owner");
        from_token_indexes.remove(
            from_token_indexes
                .iter()
                .position(|token_index_in_vec| &token_index == token_index_in_vec)
                .expect("unable to find token index in users_token"),
        );
        if from_token_indexes.len() == 0 {
            ledger().user_tokens.remove(&from);
        }

        // add the token to the new owner's tokenlist
        ledger()
            .user_tokens
            .entry(to.clone())
            .or_default()
            .push(token_index);
    }

    pub fn bearer(&self, token_identifier: &TokenIdentifier) -> AccountIdentifierReturn {
        AccountIdentifierReturn::Ok(
            ledger()
                .tokens
                .get(&into_token_index(&token_identifier))
                .expect("unable to locate token id")
                .account_identifier
                .clone(),
        )
    }

    pub fn does_token_exist(&self, token_id: u64) -> bool {
        ledger()
            .tokens
            .contains_key(&into_token_index(&token_id.to_string()))
    }

    pub fn supply(&self, _token_identifier: &TokenIdentifier) -> BalanceReturn {
        BalanceReturn::Ok(ledger().tokens.len().into())
    }

    pub fn get_all_metadata_for_user(&self, user: &User) -> Vec<TokenMetadata> {
        ledger()
            .user_tokens
            .get(user)
            .unwrap_or(&vec![])
            .iter()
            .map(|token_index| {
                ledger()
                    .tokens
                    .get(token_index)
                    .expect("unable to find token index")
                    .clone()
            })
            .collect()
    }

    pub fn metadata(&self, token_identifier: &TokenIdentifier) -> MetadataReturn {
        MetadataReturn::Ok(
            ledger()
                .tokens
                .get(&into_token_index(&token_identifier))
                .expect("unable to find token index")
                .metadata
                .clone(),
        )
    }

    #[allow(dead_code)]
    #[cfg(test)]
    pub fn clear(&mut self) {
        self.tokens.clear();
        self.user_tokens.clear();
    }
}
