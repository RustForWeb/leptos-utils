use leptos::{either::Either, prelude::*};

/// A `Show` component with typed fallback support.
#[component]
pub fn TypedFallbackShow<F, IV, W, C>(
    /// The children will be shown whenever the condition in the `when` closure returns `true`.
    children: TypedChildrenFn<C>,
    /// A closure that returns a bool that determines whether this thing runs
    when: W,
    /// A closure that returns what gets rendered if the when statement is false. By default this is the empty view.
    fallback: F,
) -> impl IntoView
where
    W: Fn() -> bool + Send + Sync + 'static,
    F: Fn() -> IV + Send + Sync + 'static,
    IV: IntoView + 'static,
    C: IntoView + 'static,
{
    let memoized_when = ArcMemo::new(move |_| when());
    let children = children.into_inner();

    move || match memoized_when.get() {
        true => Either::Left(children()),
        false => Either::Right(fallback().into_view()),
    }
}
