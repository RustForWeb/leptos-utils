use std::ops::Deref;

use leptos::prelude::{Callable, Callback};

/// A wrapper around an optional callback that provides convenient conversion and method call semantics.
#[derive(Debug, Clone)]
pub struct MaybeCallback<T: 'static>(pub Option<Callback<T>>);

impl<T> MaybeCallback<T> {
    /// Creates a new [`MaybeCallback`] from a callback.
    pub fn new(callback: impl Into<Callback<T>>) -> Self {
        Self(Some(callback.into()))
    }

    /// Returns a reference to the contained callback, if any.
    pub fn as_ref(&self) -> Option<&Callback<T>> {
        self.0.as_ref()
    }

    /// Runs the stored callback if available.
    pub fn run(&self, event: T) {
        if let Some(ref cb) = self.0 {
            cb.run(event);
        }
    }

    /// Converts this `MaybeCallback<T>` into a `MaybeCallback<U>` by applying `f`.
    pub fn map<U: 'static>(self, f: impl FnOnce(Callback<T>) -> Callback<U>) -> MaybeCallback<U> {
        MaybeCallback(self.0.map(f))
    }

    /// Returns `true` if the callback is `Some`.
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    /// Returns `true` if the callback is `None`.
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    /// Converts `MaybeCallback<T>` into a `Callback<T>` that conditionally runs the inner callback.
    pub fn as_callback(&self) -> Callback<T> {
        // Clone the inner `Option<Callback<T>>` to own it within the closure.
        let callback = self.0;
        Callback::new(move |event: T| {
            if let Some(ref cb) = callback {
                cb.run(event);
            }
        })
    }

    /// Consumes `MaybeCallback<T>` and returns a `FnMut(T)` closure that runs the callback if present.
    pub fn into_handler(self) -> impl FnMut(T) {
        move |event: T| {
            self.run(event);
        }
    }

    /// Borrows `MaybeCallback<T>` and returns a `FnMut(T)` closure that runs the callback if present.
    /// This method clones the inner callback to avoid consuming `self`.
    pub fn as_handler(&self) -> impl FnMut(T) + '_ {
        let callback = self.0;
        move |event: T| {
            if let Some(ref cb) = callback {
                cb.run(event);
            }
        }
    }
}

impl<T> Default for MaybeCallback<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T> Deref for MaybeCallback<T> {
    type Target = Option<Callback<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<MaybeCallback<T>> for Option<Callback<T>> {
    fn from(maybe: MaybeCallback<T>) -> Self {
        maybe.0
    }
}

impl<T> From<Callback<T>> for MaybeCallback<T> {
    fn from(callback: Callback<T>) -> Self {
        Self(Some(callback))
    }
}

impl<T> From<Option<Callback<T>>> for MaybeCallback<T> {
    fn from(option: Option<Callback<T>>) -> Self {
        Self(option)
    }
}

impl<T> From<Option<Option<Callback<T>>>> for MaybeCallback<T> {
    fn from(opt: Option<Option<Callback<T>>>) -> Self {
        Self(opt.flatten())
    }
}

impl<T, F> From<F> for MaybeCallback<T>
where
    T: 'static,
    F: Fn(T) + Send + Sync + 'static,
{
    fn from(f: F) -> Self {
        Self(Some(Callback::new(f)))
    }
}

impl<T, F> From<Option<F>> for MaybeCallback<T>
where
    T: 'static,
    F: Fn(T) + Send + Sync + 'static,
{
    fn from(opt: Option<F>) -> Self {
        Self(opt.map(Callback::new))
    }
}

impl<T, F> From<Option<Option<F>>> for MaybeCallback<T>
where
    T: 'static,
    F: Fn(T) + Send + Sync + 'static,
{
    fn from(opt: Option<Option<F>>) -> Self {
        Self(opt.flatten().map(Callback::new))
    }
}
