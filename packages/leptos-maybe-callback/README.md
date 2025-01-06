# Leptos Maybe Callback

Lightweight, type-safe optional callbacks for [Leptos](https://leptos.dev/). Provides a zero-cost abstraction for
conditionally handling event callbacks in your Leptos components.

## Features

- **Optional Callbacks**: Represent callbacks that may or may not exist.
- **Zero-Cost Abstraction**: Minimal overhead using Rust's `Option` and enums.
- **Seamless Integration**: Works effortlessly with Leptos signals and components.
- **Flexible Conversions**: Convert from various callback-like types, including nested `Option`s and `Fn` closures.
- **Thread-Safe & Wasm-Ready**: Implements `Send + Sync` where applicable.
- **Convenient Handler Generation**: Utilize `as_handler` and `into_handler` methods for generating event handlers.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
leptos-maybe-callback = "0.4.0"
```

## Usage Examples

### Component with Optional Callback Prop

Define a component that accepts an optional callback using `#[prop(into, optional)]`. This allows passing a closure, a
`Callback`, or omitting the prop.

```rust
use leptos::prelude::*;
use leptos_maybe_callback::MaybeCallback;

/// A button component with an optional `onclick` callback.
#[component]
#[allow(non_snake_case)]
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
#[allow(non_snake_case)]
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
use leptos::prelude::*;
use leptos_maybe_callback::MaybeCallback;

/// Parent component using `Button` with a `Callback`.
#[component]
#[allow(non_snake_case)]
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
use leptos::prelude::*;
use leptos_maybe_callback::MaybeCallback;

/// Parent component using `Button` without a callback.
#[component]
#[allow(non_snake_case)]
pub fn ButtonWithoutCallback() -> impl IntoView {
    view! {
        <div>
            <Button />
            <Button onclick=None />
        </div>
    }
}
```

## Documentation

Comprehensive (WIP + TBD) documentation is available on [docs.rs](https://docs.rs/leptos-maybe-callback).

## Testing

Run the test suite with:

```bash
cargo test
```

## Contributing

Contributions are welcome! Please submit pull requests or open issues
on [GitHub](https://github.com/RustForWeb/leptos-utils).

## Credits

The initial idea and implementation of `MaybeCallback` were contributed
by [@israelbarbara](https://github.com/israelbarbara) on Discord (
26/12/2024). [@geoffreygarrett](https://github.com/geoffreygarrett) facilitated its integration into
the [RustForWeb](https://github.com/RustForWeb) project.

## Rust For Web

Part of the [Rust For Web](https://github.com/RustForWeb) initiative to create and port web UI libraries for Rust. All
projects are free and open source.
