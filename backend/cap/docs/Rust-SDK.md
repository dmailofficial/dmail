# Cap Rust SDK Integration

Who is this for? This document is targeted towards people who like
to use the Cap service in their Rusty canisters.

Integrating Cap with your canister basically means taking care of two
processes:

1. Handshake: Initiating the connection with Cap router, and creating a root bucket.
2. Insert your data!

Yeah! It's really as simple as that.

## Handshake

The handshake method from the SDK should be called when your canister
is bootstrapped, this method only configures the handshake parameters,
the actual handshake call is deferred to the first time your canister
tries to insert some data.

The reason for that is, we found that the handshake is better to be
called in the `init` or `post_upgrade` methods, but due to technical
limitations on the Internet Computer, these methods are not capable
of making inter-canister calls (must be sync), so that's why our SDK's
`handshake` method is also a sync function, that can be called in either
of `init` or `post_upgrade`.

⚠️ The root bucket is only created after the first insert! Until an event is inserted, there'll be no root bucket id; If you fail to consider this, it might cause confusion when you try to `get_user_root_buckets`, etc. As the root bucket id will not be available or provided!

### Parameters
1. Creation cycles: The number of cycles that you allow the SDK use for creating a root bucket.
2. Router override: If you're working with a mock environment and need to use another ID for Cap router you can perform this override. Otherwise, just pass `None`.

```rust
use ic_kit::macros::*;

#[init]
fn init() {
    cap_sdk::handshake(
        1_000_000_000_000,
        None
    );
}
```

## Insert transactions

Once your canister is in a state where you know the handshake method
must have already been called, it's time for you to write data to Cap.

This example shows you how you can implement a custom detail value.

```rust
use ic_kit::macros::*;
use cap_sdk::{DetailValue, IndefiniteEventBuilder, IntoEvent};

pub struct ActionDetails {
    x: u32,
    y: u32,
}

impl IntoEvent for ActionDetails {
    fn details(&self) -> Vec<(String, DetailValue)> {
        vec![
            ("x".into(), self.x.into()),
            ("y".into(), self.y.into()),
        ]
    }
}

#[update]
fn action(x: u32, y: u32) -> u64 {
    let transaction_details = ActionDetails {
        x,
        y
    };

    let event = IndefiniteEventBuilder::new()
        .caller(ic::caller())
        .operation(String::from("action"))
        .details(transaction_details)
        .build()
        .unwrap();

    cap_sdk::insert(event).await.unwrap()
}
```