use cap_sdk_core::{Index, RootBucket, Router};
use futures::{future::LocalBoxFuture, task::AtomicWaker, Future};
use ic_kit::candid::CandidType;
use ic_kit::ic::{get_maybe, store};
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    task::{Context, Poll},
};

/// Contains data about the cap environment.
#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct CapEnv {
    pub(crate) root: RootBucket,
    pub(crate) router: Router,
}

thread_local! {
    pub(crate) static FUTURES: RefCell<Vec<LocalBoxFuture<'static, ()>>> = RefCell::new(vec![]);
}

impl CapEnv {
    /// Creates a new [`CapEnv`] with the index canister's [`Principal`] set to `index`.
    pub(crate) fn create(root: RootBucket, router: Router) -> Self {
        CapEnv { root, router }
    }

    /// Stores the [`CapEnv`] in the canister.
    pub(crate) fn store(&self) {
        store(self.clone());
    }

    pub(crate) async fn get<'a>() -> &'a Self {
        if let Some(data) = get_maybe::<CapEnv>() {
            data
        } else {
            CapEnv::await_futures().await;
            get_maybe::<CapEnv>().expect("No context created.")
        }
    }

    pub(crate) fn index(&self) -> Index {
        self.router.into()
    }

    pub(crate) async fn await_futures() {
        let futures = FUTURES.with(|futures| futures.take());

        if futures.is_empty() {
            return;
        }

        let flag = Flag::new();
        let f2 = flag.clone();

        let closure = async {
            flag.await;
        };

        CapEnv::insert_future(Box::pin(closure));

        for future in futures {
            future.await;
        }

        f2.signal();
    }

    pub(crate) fn insert_future(future: LocalBoxFuture<'static, ()>) {
        FUTURES.with(|futures| {
            futures.borrow_mut().push(future);
        });
    }

    /// Sets the [`CapEnv`] using the provided value.
    ///
    /// Used to restore the generated canister's ID after an upgrade.
    #[deprecated(note = "Use the safer cap_sdk::from_archive")]
    pub fn load_from_archive(env: CapEnv) {
        env.store();
    }

    /// Gets the [`CapEnv`].
    ///
    /// Should be used during the upgrade process of a contract canister.
    /// Call it during `pre_upgrade` to write it somewhere in stable storage.
    ///
    /// Afterwards, write it back with [`CapEnv::load_from_archive`]
    #[deprecated(note = "Use the safer cap_sdk::archive")]
    pub fn to_archive() -> Self {
        get_maybe::<CapEnv>().expect("No context created.").clone()
    }
}

struct Inner {
    waker: AtomicWaker,
    set: AtomicBool,
}

#[derive(Clone)]
struct Flag(Arc<Inner>);

impl Flag {
    pub fn new() -> Self {
        Self(Arc::new(Inner {
            waker: AtomicWaker::new(),
            set: AtomicBool::new(false),
        }))
    }

    pub fn signal(&self) {
        self.0.set.store(true, Ordering::Relaxed);
        self.0.waker.wake();
    }
}

impl Future for Flag {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // quick check to avoid registration if already done.
        if self.0.set.load(Ordering::Relaxed) {
            return Poll::Ready(());
        }

        self.0.waker.register(cx.waker());

        // Need to check condition **after** `register` to avoid a race
        // condition that would result in lost notifications.
        if self.0.set.load(Ordering::Relaxed) {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
