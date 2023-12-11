use crate::migration::v2;

use crate::write_new_users_to_cap;
use cap_common::bucket::Bucket;
use cap_common::transaction::IndefiniteEvent;
use cap_common::{TransactionId, TransactionList};
use ic_kit::{ic, Principal};

/// An in progress read from the stable storage.
pub struct InProgressReadFromStable {
    /// Index of the current event which we should start writing from.
    pub cursor: usize,
    /// The decoded information and metadata.
    pub v2: v2::Data,
    /// The transaction list that we're building.
    pub list: TransactionList,
}

impl Default for InProgressReadFromStable {
    fn default() -> Self {
        ic::trap("Not expected to be called.");
    }
}

impl InProgressReadFromStable {
    pub fn new(v2: v2::Data) -> Self {
        let list = TransactionList::new(v2.bucket.bucket.1, v2.bucket.bucket.0);

        Self {
            cursor: 0,
            v2,
            list,
        }
    }

    pub fn status(&self) -> (usize, bool) {
        (self.cursor, self.is_complete())
    }

    /// Insert a batch of transaction to the hash queue, this is used when an in-progress reader
    /// is still working.
    pub fn insert_batch(
        &mut self,
        caller: &Principal,
        transactions: Vec<IndefiniteEvent>,
    ) -> TransactionId {
        let data = &mut self.v2;
        let time = ic::time() / 1_000_000;

        if !(caller == &data.bucket.contract || data.writers.contains(caller)) {
            panic!("The method can only be invoked by one of the writers.");
        }

        let id = data.bucket.bucket.0 + (data.bucket.bucket.2.len() as u64);
        let mut new_users = Vec::new();

        for tx in transactions {
            let event = tx.to_event(time);

            for principal in event.extract_principal_ids() {
                if data.users.insert(*principal) {
                    new_users.push(*principal);
                }
            }

            data.bucket.bucket.2.push(event);
        }

        #[cfg(not(test))]
        ic_cdk::spawn(write_new_users_to_cap(
            data.cap_id,
            data.bucket.contract,
            new_users,
        ));

        // let's also make some progress.
        self.progress(100);

        id
    }

    /// Make further progress on this reader by reading `n` items from the v2 structure and putting
    /// it in the transaction list.
    pub fn progress(&mut self, n: usize) {
        let from = self.cursor;
        let to = (from + n).min(self.v2.bucket.bucket.2.len());

        for i in from..to {
            let event = self.v2.bucket.bucket.2[i].clone();
            self.list.insert(event);
            self.cursor += 1;
        }
    }

    /// Returns `true` if all of the events have been processes and we're ready to convert to a
    pub fn is_complete(&self) -> bool {
        self.cursor >= self.v2.bucket.bucket.2.len()
    }

    /// Return the number of remaining events.
    pub fn rem(&self) -> usize {
        self.v2.bucket.bucket.2.len() - self.cursor
    }

    /// If there are no more work to be done on this reader, return the Data instance.
    /// the InProgressReadFromStable should be deleted after this call.
    pub fn get_data(&mut self) -> Option<crate::Data> {
        if !self.is_complete() {
            return None;
        }

        let list = std::mem::replace(
            &mut self.list,
            TransactionList::new(Principal::management_canister(), 0),
        );
        let users = std::mem::take(&mut self.v2.users);

        Some(crate::Data {
            bucket: Bucket::with_transaction_list(list),
            users,
            cap_id: self.v2.cap_id,
            allow_migration: self.v2.allow_migration,
            writers: self.v2.writers.clone(),
        })
    }
}
