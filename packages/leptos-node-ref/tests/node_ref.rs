use leptos_node_ref::{AnyNodeRef, ToAnyNodeRef};
use leptos::{prelude::*, html};

#[test]
fn test_any_node_ref_creation() {
    let node_ref = AnyNodeRef::new();
    assert!(node_ref.get().is_none(), "New AnyNodeRef should be empty");
}

#[test]
fn test_to_any_node_ref() {
    let div_ref: NodeRef<html::Div> = NodeRef::new();
    let any_ref = div_ref.to_any();
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