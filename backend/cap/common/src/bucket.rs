use crate::did::*;
use crate::transaction::Event;
use crate::TransactionList;
use certified_vars::hashtree::{fork, fork_hash};
use certified_vars::{AsHashTree, Hash, HashTree, Map, Seq};
use ic_kit::candid::CandidType;
use ic_kit::Principal;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize)]
pub struct Bucket {
    pub bucket: TransactionList,
    buckets: Map<TransactionId, Principal>,
    next_canisters: Seq<BucketId>,
    contract: TokenContractId,
}

impl Bucket {
    /// Create a new bucket.
    pub fn new(contract: TokenContractId, offset: u64) -> Self {
        Self {
            bucket: TransactionList::new(contract, offset),
            buckets: Map::new(),
            next_canisters: Seq::new(),
            contract,
        }
    }

    pub fn with_transaction_list(list: TransactionList) -> Self {
        let contract = *list.contract_id();
        Self {
            bucket: list,
            buckets: Map::new(),
            next_canisters: Seq::new(),
            contract,
        }
    }

    pub fn get_next_canisters(&self, arg: WithWitnessArg) -> GetNextCanistersResponse {
        let witness = match arg.witness {
            false => None,
            true => Some(
                fork(
                    HashTree::Pruned(fork_hash(
                        &self.bucket.root_hash(),
                        &self.buckets.root_hash(),
                    )),
                    self.next_canisters.as_hash_tree(),
                )
                .into(),
            ),
        };

        let canisters = self.next_canisters.as_vec().clone();

        GetNextCanistersResponse { canisters, witness }
    }

    pub fn get_transaction(&self, arg: WithIdArg) -> GetTransactionResponse {
        let witness = match arg.witness {
            false => None,
            true => Some(
                fork(
                    fork(
                        self.bucket.witness_transaction(arg.id),
                        HashTree::Pruned(self.buckets.root_hash()),
                    ),
                    HashTree::Pruned(self.next_canisters.root_hash()),
                )
                .into(),
            ),
        };

        let event = self.bucket.get_transaction(arg.id);

        // TODO(qti3e) We're going to be in this release, take another look.
        // We are not multi-canistered yet.
        GetTransactionResponse::Found(event.cloned(), witness)
    }

    pub fn get_transactions(&self, arg: GetTransactionsArg) -> GetTransactionsResponseBorrowed {
        let page = arg
            .page
            .unwrap_or_else(|| self.bucket.last_page_for_contract(&self.contract));

        let witness = match arg.witness {
            false => None,
            true => Some(
                fork(
                    fork(
                        self.bucket
                            .witness_transactions_for_contract(&self.contract, page),
                        HashTree::Pruned(self.buckets.root_hash()),
                    ),
                    HashTree::Pruned(self.next_canisters.root_hash()),
                )
                .into(),
            ),
        };

        let events = self
            .bucket
            .get_transactions_for_contract(&self.contract, page);

        GetTransactionsResponseBorrowed {
            data: events,
            page,
            witness,
        }
    }

    pub fn get_user_transactions(
        &self,
        arg: GetUserTransactionsArg,
    ) -> GetTransactionsResponseBorrowed {
        let page = arg
            .page
            .unwrap_or_else(|| self.bucket.last_page_for_user(&arg.user));

        let witness = match arg.witness {
            false => None,
            true => Some(
                fork(
                    fork(
                        self.bucket.witness_transactions_for_user(&arg.user, page),
                        HashTree::Pruned(self.buckets.root_hash()),
                    ),
                    HashTree::Pruned(self.next_canisters.root_hash()),
                )
                .into(),
            ),
        };

        let events = self.bucket.get_transactions_for_user(&arg.user, page);

        GetTransactionsResponseBorrowed {
            data: events,
            page,
            witness,
        }
    }

    pub fn get_token_transactions(
        &self,
        arg: GetTokenTransactionsArg,
    ) -> GetTransactionsResponseBorrowed {
        let page = arg
            .page
            .unwrap_or_else(|| self.bucket.last_page_for_token(&arg.token_id));

        let witness = match arg.witness {
            false => None,
            true => Some(
                fork(
                    fork(
                        self.bucket
                            .witness_transactions_for_token(&arg.token_id, page),
                        HashTree::Pruned(self.buckets.root_hash()),
                    ),
                    HashTree::Pruned(self.next_canisters.root_hash()),
                )
                .into(),
            ),
        };

        let events = self.bucket.get_transactions_for_token(&arg.token_id, page);

        GetTransactionsResponseBorrowed {
            data: events,
            page,
            witness,
        }
    }

    pub fn get_bucket_for(&self, arg: WithIdArg) -> GetBucketResponse {
        let id_witness = self.buckets.witness(&arg.id);
        let id = id_witness
            .get_leaf_values()
            .get(0)
            .map(|bytes| Principal::from_slice(bytes))
            .unwrap_or_else(ic_kit::ic::id);

        let witness = match arg.witness {
            false => None,
            true => Some(
                fork(
                    fork(HashTree::Pruned(self.bucket.root_hash()), id_witness),
                    HashTree::Pruned(self.next_canisters.root_hash()),
                )
                .into(),
            ),
        };

        GetBucketResponse {
            canister: id,
            witness,
        }
    }

    pub fn size(&self) -> u64 {
        self.bucket.size()
    }

    pub fn contract_id(&self) -> &Principal {
        &self.contract
    }

    #[inline]
    pub fn insert(&mut self, event: Event) -> u64 {
        self.bucket.insert(event)
    }

    #[inline]
    pub fn set_next_canisters(&mut self, canisters: Vec<Principal>) {
        self.next_canisters = canisters.into();
    }
}

impl AsHashTree for Bucket {
    fn root_hash(&self) -> Hash {
        fork_hash(
            &fork_hash(&self.bucket.root_hash(), &self.buckets.root_hash()),
            &self.next_canisters.root_hash(),
        )
    }

    fn as_hash_tree(&self) -> HashTree<'_> {
        fork(
            fork(self.bucket.as_hash_tree(), self.buckets.as_hash_tree()),
            self.next_canisters.as_hash_tree(),
        )
    }
}
