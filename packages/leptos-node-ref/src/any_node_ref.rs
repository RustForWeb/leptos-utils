use std::marker::PhantomData;

use leptos::{
    attr::{Attribute, NextAttribute},
    html::ElementType,
    prelude::{
        DefinedAt, Get, NodeRef, ReadUntracked, RwSignal, Set, Track,
        guards::{Derefable, ReadGuard},
    },
    tachys::{html::node_ref::NodeRefContainer, renderer::types::Element},
};
use send_wrapper::SendWrapper;

/// A reactive reference to a DOM node that can be used with the `node_ref` attribute.
#[derive(Debug)]
pub struct AnyNodeRef(RwSignal<Option<SendWrapper<Element>>>);

impl AnyNodeRef {
    /// Creates a new [`AnyNodeRef`].
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

impl<T: ElementType> From<NodeRef<T>> for AnyNodeRef
where
    NodeRef<T>: IntoAnyNodeRef,
{
    fn from(value: NodeRef<T>) -> Self {
        value.into_any()
    }
}

impl<E: ElementType> NodeRefContainer<E> for AnyNodeRef {
    fn load(self, el: &Element) {
        self.0.set(Some(SendWrapper::new(el.clone())));
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

/// Allows converting any node reference into our type-erased [`AnyNodeRef`].
pub trait IntoAnyNodeRef {
    /// Converts `self` into an [`AnyNodeRef`].
    fn into_any(self) -> AnyNodeRef;
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

/// Attribute wrapper for node references that allows conditional rendering across elements.
///
/// Useful when distributing node references across multiple rendering branches.
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

    fn add_any_attr<NewAttr: Attribute>(self, new_attr: NewAttr) -> Self::Output<NewAttr> {
        (self, new_attr)
    }
}

/// Constructs an attribute to attach an [`AnyNodeRef`] to an element.
///
/// Enables adding node references in conditional/dynamic rendering branches.
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

#[cfg(test)]
mod tests {
    use leptos::{html, prelude::*};

    use super::{any_node_ref, prelude::*};

    #[test]
    fn test_any_node_ref_creation() {
        let node_ref = AnyNodeRef::new();
        assert!(node_ref.get().is_none(), "New AnyNodeRef should be empty");
    }

    #[test]
    fn test_to_any_node_ref() {
        let div_ref: NodeRef<html::Div> = NodeRef::new();
        let any_ref = div_ref.into_any();
        assert!(
            any_ref.get().is_none(),
            "Converted AnyNodeRef should be initially empty"
        );
    }

    #[test]
    fn test_clone_and_copy() {
        let node_ref = AnyNodeRef::new();
        let cloned_ref = node_ref;
        let _copied_ref = cloned_ref; // Should be copyable
        assert!(
            cloned_ref.get().is_none(),
            "Cloned AnyNodeRef should be empty"
        );
    }

    #[test]
    fn test_default() {
        let node_ref = AnyNodeRef::default();
        assert!(
            node_ref.get().is_none(),
            "Default AnyNodeRef should be empty"
        );
    }

    #[test]
    fn test_into_any_node_ref_trait() {
        let div_ref: NodeRef<html::Div> = NodeRef::new();
        let _any_ref: AnyNodeRef = div_ref.into_any();

        let input_ref: NodeRef<html::Input> = NodeRef::new();
        let _any_input_ref: AnyNodeRef = input_ref.into_any();
    }

    #[test]
    fn test_from_node_ref() {
        let div_ref: NodeRef<html::Div> = NodeRef::new();
        let _any_ref: AnyNodeRef = div_ref.into();
    }

    #[test]
    fn test_any_node_ref_attr() {
        let node_ref = AnyNodeRef::new();
        let _attr = any_node_ref::<html::Div, _>(node_ref);
    }

    #[test]
    fn test_defined_at() {
        let node_ref = AnyNodeRef::new();
        assert!(node_ref.defined_at().is_some());
    }

    #[test]
    fn test_track_and_untracked() {
        let node_ref = AnyNodeRef::new();
        // Just testing that these don't panic
        node_ref.track();
        let _untracked = node_ref.try_read_untracked();
    }

    #[test]
    fn test_into_any_identity() {
        let node_ref = AnyNodeRef::new();
        let same_ref = node_ref.into_any();

        // Instead of checking pointer equality, we should verify:
        // 1. Both refs are initially empty
        assert!(node_ref.get().is_none());
        assert!(same_ref.get().is_none());

        // 2. When we set one, both should reflect the change
        // (This would require a mock Element to test properly)

        // 3. They should have the same defined_at location
        assert_eq!(node_ref.defined_at(), same_ref.defined_at());
    }
}
