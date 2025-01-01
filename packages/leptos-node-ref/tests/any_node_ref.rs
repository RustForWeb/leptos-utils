use leptos::{html, prelude::*};
use leptos_node_ref::{any_node_ref, prelude::*};

#[test]
fn test_any_node_ref_creation() {
    let node_ref = AnyNodeRef::new();
    assert!(node_ref.get().is_none(), "New AnyNodeRef should be empty");
}

#[test]
fn test_to_any_node_ref() {
    let div_ref: NodeRef<html::Div> = NodeRef::new();
    let any_ref = div_ref.into_any();
    assert!(any_ref.get().is_none(), "Converted AnyNodeRef should be initially empty");
}

#[test]
fn test_clone_and_copy() {
    let node_ref = AnyNodeRef::new();
    let cloned_ref = node_ref;
    let _copied_ref = cloned_ref; // Should be copyable
    assert!(cloned_ref.get().is_none(), "Cloned AnyNodeRef should be empty");
}

#[test]
fn test_default() {
    let node_ref = AnyNodeRef::default();
    assert!(node_ref.get().is_none(), "Default AnyNodeRef should be empty");
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