# Leptos Maybe Callback

Optional callbacks for [Leptos](https://leptos.dev/).

## Documentation

Documentation for the crate is available on [Docs.rs](https://docs.rs/):

-   [`leptos-maybe-callback`](https://docs.rs/leptos-maybe-callback/latest/leptos_maybe_callback/)

## Example

### Component with Optional Callback Prop

Define a component that accepts an optional callback using `#[prop(into, optional)]`. This allows passing a closure, a
`Callback`, or omitting the prop.

```rust
use leptos::{ev::MouseEvent, prelude::*};
use leptos_maybe_callback::MaybeCallback;

/// A button component with an optional `onclick` callback.
#[component]
pub fn Button(
    #[prop(into, optional)]
    onclick: MaybeCallback<MouseEvent>,
) -> impl IntoView {
    view! {
        <button on:click=onclick.into_handler()>
            "Click me"
        </button>
    }
}
```

### Using the Component with a Closure

Use the `Button` component and provide a closure for the `onclick` prop.

```rust
use leptos::prelude::*;
use leptos_maybe_callback::MaybeCallback;

/// Parent component using `Button` with a closure.
#[component]
pub fn ButtonWithClosure() -> impl IntoView {
    view! {
        <div>
            <Button onclick=|_| log::info!("Clicked via closure!") />
            <Button />
        </div>
    }
}
```

### Using the Component with a `Callback`

Alternatively, pass a `Callback` as the `onclick` prop.

```rust
use leptos::{ev::MouseEvent, prelude::*};
use leptos_maybe_callback::MaybeCallback;

/// Parent component using `Button` with a `Callback`.
#[component]
pub fn ButtonWithCallback() -> impl IntoView {
    let on_click = Callback::new(|event: MouseEvent| {
        log::info!("Clicked with event: {:?}", event);
    });

    view! {
        <div>
            <Button onclick=on_click />
            <Button />
        </div>
    }
}
```

### Omitting the Callback

If no callback is needed, omit the `onclick` prop or pass `None`.

```rust
use leptos::{ev::MouseEvent, prelude::*};
use leptos_maybe_callback::MaybeCallback;

/// Parent component using `Button` without a callback.
#[component]
pub fn ButtonWithoutCallback() -> impl IntoView {
    view! {
        <div>
            <Button />
            <Button onclick={None::<Callback<MouseEvent>>} />
        </div>
    }
}
```

## Rust For Web

The Leptos Maybe Callback project is part of [Rust For Web](https://github.com/RustForWeb).

[Rust For Web](https://github.com/RustForWeb) creates and ports web UI libraries for Rust. All projects are free and open source.
