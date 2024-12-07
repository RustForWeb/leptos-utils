use leptos::{
    prelude::{
        guards::{Derefable, ReadGuard},
        DefinedAt, ReadUntracked, RwSignal, Set, Track,
    },
    tachys::{html::node_ref::NodeRefContainer, renderer::types::Element},
};
use send_wrapper::SendWrapper;

/// A reactive reference to a DOM node that can be used with the `node_ref` attribute.
#[derive(Debug)]
pub struct AnyNodeRef(RwSignal<Option<SendWrapper<Element>>>);

impl AnyNodeRef {
    /// Creates a new node reference.
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

macro_rules! impl_html_any_node_ref {
    ($($element:ident),*,) => {
        $(impl NodeRefContainer<leptos::html::$element> for AnyNodeRef {
            fn load(self, el: &Element) {
                // safe to construct SendWrapper here, because it will only run in the browser
                // so it will always be accessed or dropped from the main thread
                self.0.set(Some(SendWrapper::new(el.clone())));
            }
        })*
    };
}

macro_rules! impl_math_any_node_ref {
    ($($element:ident),*,) => {
        $(impl NodeRefContainer<leptos::math::$element> for AnyNodeRef {
            fn load(self, el: &Element) {
                // safe to construct SendWrapper here, because it will only run in the browser
                // so it will always be accessed or dropped from the main thread
                self.0.set(Some(SendWrapper::new(el.clone())));
            }
        })*
    };
}

macro_rules! impl_svg_any_node_ref {
    ($($element:ident),*,) => {
        $(impl NodeRefContainer<leptos::svg::$element> for AnyNodeRef {
            fn load(self, el: &Element) {
                // safe to construct SendWrapper here, because it will only run in the browser
                // so it will always be accessed or dropped from the main thread
                self.0.set(Some(SendWrapper::new(el.clone())));
            }
        })*
    };
}

impl_html_any_node_ref!(
    A, Abbr, Address, Area, Article, Aside, Audio, B, Base, Bdi, Bdo, Blockquote, Body, Br, Button,
    Canvas, Caption, Cite, Code, Col, Colgroup, Data, Datalist, Dd, Del, Details, Dfn, Dialog, Div,
    Dl, Dt, Em, Embed, Fieldset, Figcaption, Figure, Footer, Form, H1, H2, H3, H4, H5, H6, Head,
    Header, Hgroup, Hr, Html, I, Iframe, Img, Input, Ins, Kbd, Label, Legend, Li, Link, Main, Map,
    Mark, Menu, Meta, Meter, Nav, Noscript, Object, Ol, Optgroup, Option_, Output, P, Picture,
    Portal, Pre, Progress, Q, Rp, Rt, Ruby, S, Samp, Script, Search, Section, Select, Slot, Small,
    Source, Span, Strong, Style, Sub, Summary, Sup, Table, Tbody, Td, Template, Textarea, Tfoot,
    Th, Thead, Time, Title, Tr, Track, U, Ul, Var, Video, Wbr,
);

impl_math_any_node_ref!(
    Math,
    Mi,
    Mn,
    Mo,
    Ms,
    Mspace,
    Mtext,
    Menclose,
    Merror,
    Mfenced,
    Mfrac,
    Mpadded,
    Mphantom,
    Mroot,
    Mrow,
    Msqrt,
    Mstyle,
    Mmultiscripts,
    Mover,
    Mprescripts,
    Msub,
    Msubsup,
    Msup,
    Munder,
    Munderover,
    Mtable,
    Mtd,
    Mtr,
    Maction,
    Annotation,
    Semantics,
);

impl_svg_any_node_ref!(
    A,
    Animate,
    AnimateMotion,
    AnimateTransform,
    Circle,
    ClipPath,
    Defs,
    Desc,
    Discard,
    Ellipse,
    FeBlend,
    FeColorMatrix,
    FeComponentTransfer,
    FeComposite,
    FeConvolveMatrix,
    FeDiffuseLighting,
    FeDisplacementMap,
    FeDistantLight,
    FeDropShadow,
    FeFlood,
    FeFuncA,
    FeFuncB,
    FeFuncG,
    FeFuncR,
    FeGaussianBlur,
    FeImage,
    FeMerge,
    FeMergeNode,
    FeMorphology,
    FeOffset,
    FePointLight,
    FeSpecularLighting,
    FeSpotLight,
    FeTile,
    FeTurbulence,
    Filter,
    ForeignObject,
    G,
    Hatch,
    Hatchpath,
    Image,
    Line,
    LinearGradient,
    Marker,
    Mask,
    Metadata,
    Mpath,
    Path,
    Pattern,
    Polygon,
    Polyline,
    RadialGradient,
    Rect,
    Script,
    Set,
    Stop,
    Style,
    Svg,
    Switch,
    Symbol,
    Text,
    TextPath,
    Title,
    Tspan,
    View,
);
