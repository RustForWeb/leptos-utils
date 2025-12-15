use std::fmt::{self, Display};

use leptos::{ev::MouseEvent, prelude::*};
use leptos_node_ref::AnyNodeRef;
use leptos_struct_component::StructComponent;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum BoxAs {
    #[default]
    Div,
    Span,
}

impl Display for BoxAs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BoxAs::Div => "div",
                BoxAs::Span => "span",
            }
        )
    }
}

#[derive(Clone, StructComponent)]
pub struct BoxChildProps {
    #[struct_component(dynamic_tag = [BoxAs::Div, BoxAs::Span])]
    pub r#as: BoxAs,
    pub node_ref: AnyNodeRef,
    // pub attributes: Attributes,

    // Global attributes
    pub class: MaybeProp<String>,
    pub id: MaybeProp<String>,
    pub style: MaybeProp<String>,
}

#[component]
pub fn Box(
    #[prop(into, optional)] r#as: BoxAs,

    // Global attributes
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    // #[prop(into, optional)] attributes: Attributes,
    #[prop(into, optional)] as_child: Option<Callback<BoxChildProps, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let child_props = BoxChildProps {
        r#as,
        node_ref,
        // attributes: attributes.clone(),

        // Global attributes
        class,
        id,
        style,
    };

    if let Some(as_child) = as_child {
        as_child.run(child_props)
    } else {
        child_props.render(children)
    }
}

#[derive(Clone, StructComponent)]
#[struct_component(tag = "img", no_children = true)]
pub struct ImageChildProps {
    pub node_ref: AnyNodeRef,
    // pub attributes: Attributes,

    // Global attributes
    pub class: MaybeProp<String>,
    pub id: MaybeProp<String>,
    pub style: MaybeProp<String>,

    // Event handler attributes
    pub onclick: Option<Callback<MouseEvent>>,
}

#[component]
pub fn Image(
    // Global attributes
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,

    // Event handler attributes
    #[prop(into, optional)] on_click: Option<Callback<MouseEvent>>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    // #[prop(into, optional)] attributes: Attributes,
    #[prop(into, optional)] as_child: Option<Callback<ImageChildProps, AnyView>>,
) -> impl IntoView {
    let child_props = ImageChildProps {
        node_ref,
        // attributes: attributes.clone().with_defaults([("alt", "Image")]),

        // Global attributes
        class,
        id,
        style,

        // Event handler attributes
        onclick: on_click,
    };

    if let Some(as_child) = as_child {
        as_child.run(child_props)
    } else {
        child_props.render()
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Box>
            <Image
                // attributes={[
                //     ("src", "https://picsum.photos/id/10/200/300")
                // ]}
            />
        </Box>
    }
}
