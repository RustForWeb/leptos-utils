use leptos::{
    either::Either,
    prelude::*,
};

/// We need our own show instead of leptos' Show because attribute spreading does not work
/// across AnyView as of 0.7.2, which is required here.
#[component]
#[allow(non_snake_case)]
pub fn TypedFallbackShow<F, IV, W, C>(
    children: TypedChildrenFn<C>,
    when: W,
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