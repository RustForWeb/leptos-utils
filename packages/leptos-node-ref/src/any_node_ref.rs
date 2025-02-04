use std::marker::PhantomData;
use leptos::{
    attr::{Attribute, NextAttribute},
    html::ElementType,
    prelude::{
        guards::{Derefable, ReadGuard},
        DefinedAt, Get, NodeRef, ReadUntracked, RwSignal, Set, Track,
    },
    tachys::{
        html::node_ref::NodeRefContainer,
        renderer::types::Element,
    },
};
use send_wrapper::SendWrapper;

/// A reactive reference to a DOM node that can be used with the `node_ref` attribute.
#[derive(Debug)]
pub struct AnyNodeRef(RwSignal<Option<SendWrapper<Element>>>);

impl AnyNodeRef {
    /// Creates a new `AnyNodeRef`.
    #[track_caller]
    pub fn new() -> Self {
        Self(RwSignal::new(None))
    }
}

impl Default for AnyNodeRef {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for AnyNodeRef {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for AnyNodeRef {}

impl DefinedAt for AnyNodeRef {
    fn defined_at(&self) -> Option<&'static std::panic::Location<'static>> {
        self.0.defined_at()
    }
}

impl ReadUntracked for AnyNodeRef {
    type Value = ReadGuard<Option<Element>, Derefable<Option<Element>>>;

    fn try_read_untracked(&self) -> Option<Self::Value> {
        Some(ReadGuard::new(Derefable(
            self.0.try_read_untracked()?.as_deref().cloned(),
        )))
    }
}

impl Track for AnyNodeRef {
    fn track(&self) {
        self.0.track();
    }
}

/// Allows converting any node reference into our type-erased `AnyNodeRef`.
pub trait IntoAnyNodeRef {
    /// Converts `self` into an `AnyNodeRef`.
    fn into_any(self) -> AnyNodeRef;
}

impl<E: ElementType> NodeRefContainer<E> for AnyNodeRef {
    fn load(self, el: &Element) {
        self.0.set(Some(SendWrapper::new(el.clone())));
    }
}

impl<E> IntoAnyNodeRef for NodeRef<E>
where
    E: ElementType,
    E::Output: AsRef<Element>,
    NodeRef<E>: Get<Value = Option<E::Output>>,
{
    fn into_any(self) -> AnyNodeRef {
        let any_ref = AnyNodeRef::new();
        if let Some(element) = self.get() {
            NodeRefContainer::<E>::load(any_ref, element.as_ref());
        }
        any_ref
    }
}

impl IntoAnyNodeRef for AnyNodeRef {
    fn into_any(self) -> AnyNodeRef {
        self
    }
}

impl<T: ElementType> From<NodeRef<T>> for AnyNodeRef
where
    NodeRef<T>: IntoAnyNodeRef,
{
    fn from(value: NodeRef<T>) -> Self {
        value.into_any()
    }
}

/// Attribute wrapper for node refs that allows conditional rendering across elements.
///
/// Useful when distributing node refs across multiple rendering branches.
#[derive(Debug)]
pub struct AnyNodeRefAttr<E, C> {
    container: C,
    ty: PhantomData<E>,
}

impl<E, C> Clone for AnyNodeRefAttr<E, C>
where
    C: Clone,
{
    fn clone(&self) -> Self {
        Self {
            container: self.container.clone(),
            ty: PhantomData,
        }
    }
}

impl<E, C> Attribute for AnyNodeRefAttr<E, C>
where
    E: ElementType + 'static,
    C: NodeRefContainer<E> + Clone + 'static,
    Element: PartialEq,
{
    const MIN_LENGTH: usize = 0;
    type State = Element;
    type AsyncOutput = Self;
    type Cloneable = Self;
    type CloneableOwned = Self;

    #[inline(always)]
    fn html_len(&self) -> usize {
        0
    }

    fn to_html(
        self,
        _buf: &mut String,
        _class: &mut String,
        _style: &mut String,
        _inner_html: &mut String,
    ) {
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
        self.container.load(el);
        el.clone()
    }

    fn build(self, el: &Element) -> Self::State {
        self.container.load(el);
        el.clone()
    }

    fn rebuild(self, state: &mut Self::State) {
        self.container.load(state);
    }

    fn into_cloneable(self) -> Self::Cloneable {
        self
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        self
    }

    fn dry_resolve(&mut self) {}

    async fn resolve(self) -> Self::AsyncOutput {
        self
    }
}

impl<E, C> NextAttribute for AnyNodeRefAttr<E, C>
where
    E: ElementType + 'static,
    C: NodeRefContainer<E> + Clone + 'static,
    Element: PartialEq,
{
    type Output<NewAttr: Attribute> = (Self, NewAttr);

    fn add_any_attr<NewAttr: Attribute>(
        self,
        new_attr: NewAttr,
    ) -> Self::Output<NewAttr> {
        (self, new_attr)
    }
}

/// Constructs an attribute to attach an `AnyNodeRef` to an element.
///
/// Enables adding node refs in conditional/dynamic rendering branches.
pub fn any_node_ref<E, C>(container: C) -> AnyNodeRefAttr<E, C>
where
    E: ElementType,
    C: NodeRefContainer<E>,
{
    AnyNodeRefAttr {
        container,
        ty: PhantomData,
    }
}

pub mod prelude {
    pub use super::*;
    pub use AnyNodeRef;
    pub use IntoAnyNodeRef;
    pub use any_node_ref;
}
