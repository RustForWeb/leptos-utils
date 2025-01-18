use leptos::prelude::{Callback, Callable};
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Arc;
use leptos_maybe_callback::*;

/// Tests the default value of `MaybeCallback`, expecting it to be `None`.
#[test]
fn test_default() {
    let maybe_callback: MaybeCallback<()> = Default::default();
    assert!(maybe_callback.is_none(), "Expected MaybeCallback to be None by default.");
}

/// Tests creating a `MaybeCallback` from a `Some(Callback)`, expecting it to contain `Some`.
#[test]
fn test_from_some_callback() {
    let was_called = Arc::new(AtomicBool::new(false));
    let was_called_clone = Arc::clone(&was_called);

    let cb = Callback::new(move |_: bool| {
        was_called_clone.store(true, Ordering::SeqCst);
    });

    let maybe = MaybeCallback::from(Some(cb));
    assert!(maybe.is_some(), "Expected MaybeCallback to be Some.");

    // Execute the callback to ensure it works
    maybe.run(true);
    assert!(was_called.load(Ordering::SeqCst), "Callback was not called.");
}

/// Tests creating a `MaybeCallback` from `None`, expecting it to be `None`.
#[test]
fn test_from_none_callback() {
    let maybe: MaybeCallback<()> = MaybeCallback::from(None::<Callback<()>>);
    assert!(maybe.is_none(), "Expected MaybeCallback to be None when initialized with None.");
}

/// Tests the `run` method when `MaybeCallback` contains `Some(Callback)`.
#[test]
fn test_run_some() {
    let counter = Arc::new(AtomicI32::new(0));
    let counter_clone = Arc::clone(&counter);

    let cb = Callback::new(move |val: i32| {
        counter_clone.fetch_add(val, Ordering::SeqCst);
    });

    let maybe = MaybeCallback::from(Some(cb));
    maybe.run(5);
    assert_eq!(
        counter.load(Ordering::SeqCst),
        5,
        "Counter should have been incremented by 5."
    );
}

/// Tests the `run` method when `MaybeCallback` is `None`, ensuring no action is taken.
#[test]
fn test_run_none() {
    let counter = Arc::new(AtomicI32::new(0));
    let maybe: MaybeCallback<i32> = MaybeCallback::from(None::<Callback<i32>>);
    maybe.run(5);
    // Should remain unchanged
    assert_eq!(
        counter.load(Ordering::SeqCst),
        0,
        "Counter should remain unchanged when MaybeCallback is None."
    );
}

/// Tests the `map` method on a `MaybeCallback` containing `Some(Callback)`.
#[test]
fn test_map_some() {
    let was_called = Arc::new(AtomicBool::new(false));
    let was_called_clone = Arc::clone(&was_called);

    let original_cb = Callback::new(move |val: i32| {
        assert_eq!(val, 42, "Callback received incorrect value.");
        was_called_clone.store(true, Ordering::SeqCst);
    });

    let maybe = MaybeCallback::from(Some(original_cb));

    // Map i32 -> &str
    let new_maybe = maybe.map(|cb| Callback::new(move |_: &str| {
        cb.run(42); // calls original callback
    }));

    assert!(new_maybe.is_some(), "Mapped MaybeCallback should be Some.");

    // Execute the mapped callback
    new_maybe.run("Hello");
    assert!(
        was_called.load(Ordering::SeqCst),
        "Mapped callback was not called."
    );
}

/// Tests the `map` method on a `MaybeCallback` that is `None`, ensuring it remains `None`.
#[test]
fn test_map_none() {
    let maybe: MaybeCallback<i32> = MaybeCallback::from(None::<Callback<i32>>);
    let new_maybe = maybe.map(|_cb| {
        // This closure should never be called
        Callback::new(|val: &str| println!("val: {}", val))
    });
    assert!(
        new_maybe.is_none(),
        "Mapped MaybeCallback should remain None when original is None."
    );
}

/// Tests the `as_handler` method by generating multiple handlers and ensuring they work independently.
#[test]
fn test_as_handler_multiple() {
    let counter1 = Arc::new(AtomicI32::new(0));
    let counter1_clone = Arc::clone(&counter1);

    let counter2 = Arc::new(AtomicI32::new(0));
    let counter2_clone = Arc::clone(&counter2);

    let cb = Callback::new(move |val: i32| {
        counter1_clone.fetch_add(val, Ordering::SeqCst);
    });

    let maybe = MaybeCallback::from(Some(cb));
    let mut handler1 = maybe.as_handler();

    // Create another handler with a different callback
    let cb2 = Callback::new(move |val: i32| {
        counter2_clone.fetch_add(val, Ordering::SeqCst);
    });
    let maybe2 = MaybeCallback::from(Some(cb2));
    let mut handler2 = maybe2.as_handler();

    handler1(10);
    handler2(20);

    assert_eq!(
        counter1.load(Ordering::SeqCst),
        10,
        "First handler should have incremented counter1 by 10."
    );
    assert_eq!(
        counter2.load(Ordering::SeqCst),
        20,
        "Second handler should have incremented counter2 by 20."
    );
}

/// Tests the `as_callback` method to ensure it returns a `Callback` that conditionally executes.
#[test]
fn test_as_callback_some() {
    let was_called = Arc::new(AtomicBool::new(false));
    let was_called_clone = Arc::clone(&was_called);

    let cb = Callback::new(move |_| {
        was_called_clone.store(true, Ordering::SeqCst);
    });

    let maybe = MaybeCallback::from(Some(cb));
    let callback = maybe.as_callback();
    callback.run(());

    assert!(
        was_called.load(Ordering::SeqCst),
        "Callback should have been executed."
    );
}

/// Tests the `as_callback` method when `MaybeCallback` is `None`, ensuring it does nothing.
#[test]
fn test_as_callback_none() {
    let maybe: MaybeCallback<()> = MaybeCallback::from(None::<Callback<()>>);
    let callback = maybe.as_callback();
    // Should not panic or do anything
    callback.run(());
}

/// Tests the `into_handler` method by consuming the `MaybeCallback` and ensuring it cannot be used afterward.
#[test]
fn test_into_handler() {
    let counter = Arc::new(AtomicI32::new(0));
    let counter_clone = Arc::clone(&counter);

    let cb = Callback::new(move |val: i32| {
        counter_clone.fetch_add(val, Ordering::SeqCst);
    });

    let maybe = MaybeCallback::from(Some(cb));
    let mut handler = maybe.into_handler();
    handler(15);

    assert_eq!(
        counter.load(Ordering::SeqCst),
        15,
        "Counter should have been incremented by 15."
    );

    // Since `maybe` is consumed, attempting to use it should result in a compile-time error.
    // Uncommenting the following lines should cause a compilation error.
    //
    // maybe.run(10); // Error: use of moved value: `maybe`
}

/// Tests the deprecated `generate_handler` function to ensure it still works as expected.
#[test]
#[allow(deprecated)]
fn test_generate_handler() {
    let counter = Arc::new(AtomicI32::new(0));
    let counter_clone = Arc::clone(&counter);

    let cb = Callback::new(move |val: i32| {
        counter_clone.fetch_add(val, Ordering::SeqCst);
    });

    let mut handler = generate_handler(cb);
    handler(10);
    handler(5);
    assert_eq!(
        counter.load(Ordering::SeqCst),
        15,
        "Counter should have been incremented by 15 using generate_handler."
    );
}

/// Tests the deprecated `Handler::from` method to ensure it still works as expected.
#[test]
#[allow(deprecated)]
fn test_handler_from() {
    let counter = Arc::new(AtomicI32::new(0));
    let counter_clone = Arc::clone(&counter);

    let cb = Callback::new(move |val: i32| {
        counter_clone.fetch_add(val, Ordering::SeqCst);
    });

    let mut handler_fn = Handler::from(MaybeCallback::from(Some(cb)));
    handler_fn(7);
    handler_fn(3);
    assert_eq!(
        counter.load(Ordering::SeqCst),
        10,
        "Counter should have been incremented by 10 using Handler::from."
    );
}
