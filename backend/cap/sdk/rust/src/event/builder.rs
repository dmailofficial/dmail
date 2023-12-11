use cap_sdk_core::transaction::{DetailValue, IndefiniteEvent};
use ic_kit::Principal;

use super::IntoEvent;

/// Constructs an [`IndefiniteEvent`].
///
/// If the type used for a call to `IndefiniteEventBuilder.details` has a non-default
/// implementation of [`IntoEvent::operation`], it will override the current operation
/// and cause any call to `IndefiniteEventBuilder.operation` to panic.
///
/// # Examples
///
/// ### Create an event with no details
/// Any [`IntoEvent`] type can be substituted into the `details` call.
///
/// ⚠️ Note the above statement if that type uses the non-default
/// implementation of [`IntoEvent::operation`]. ⚠️
///
/// ```rust
//# use cap_sdk_core::transaction::{EventStatus, IndefiniteEvent};
//# use ic_kit::Principal;
//# use crate::IndefiniteEventBuilder;
//# fn wrapper() -> Result<(), ()> {
/// let event = IndefiniteEventBuilder::new()
///     .caller(Principal::anonymous())
///     .status(EventStatus::Completed)
///     .operation("Foo")
///     .details(vec![])
///     .build()?;
//# Ok(())
//# }
///```
#[derive(Default, Clone)]
pub struct IndefiniteEventBuilder {
    caller: Option<Principal>,
    operation: Option<String>,
    details: Vec<(String, DetailValue)>,
    operation_from_event: bool,
}

impl IndefiniteEventBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the caller of the event.
    #[inline(always)]
    pub fn caller(mut self, caller: Principal) -> Self {
        self.caller = Some(caller);

        self
    }

    /// Sets the operation of the event.
    ///
    /// # Panics
    /// Panics if the operation is already set from an [`IntoEvent`] type which
    /// sets the operation.
    #[inline(always)]
    pub fn operation(mut self, operation: impl Into<String>) -> Self {
        if self.operation_from_event {
            panic!("Tried to set operation after it was set from an `IntoEvent` type")
        }

        self.operation = Some(operation.into());

        self
    }

    /// Sets the details of the event.
    ///
    /// You can combine multiple `IntoEvent` types into one [`IndefiniteEvent`]
    /// with this method as long as only one of them has a non-default implementation
    /// of [`IntoEvent::operation`].
    ///
    /// # Panics
    /// Panics if `operation` has already been set by an [`IntoEvent::operation`] call.
    #[inline(always)]
    pub fn details(mut self, details: impl IntoEvent) -> Self {
        self.details.append(&mut details.details());

        if self.operation_from_event {
            panic!("Cannot combine two `IntoEvent` types with unique operations.")
        }

        if let Some(operation) = details.operation() {
            self.operation_from_event = true;
            self.operation = Some(operation.to_owned());
        }

        self
    }

    /// Builds an [`IndefiniteEvent`] from the builder.
    ///
    /// # Panics
    /// Panics if caller, operation, or details has not
    /// already been set.
    #[inline(always)]
    pub fn build(self) -> Result<IndefiniteEvent, ()> {
        Ok(IndefiniteEvent {
            caller: self.caller.unwrap(),
            operation: self.operation.unwrap(),
            details: self.details,
        })
    }
}
