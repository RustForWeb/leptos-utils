//! Define [Leptos](https://leptos.dev/) components using structs.

extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    AttrStyle, Attribute, Data, DeriveInput, Expr, ExprArray, GenericArgument, Ident, LitBool,
    LitStr, Meta, PathArguments, Type, parse_macro_input, spanned::Spanned,
};

#[derive(Debug, Default)]
struct StructComponentAttrArgs {
    tag: Option<String>,
    dynamic_tag: Option<Vec<(Expr, String)>>,
    no_children: Option<bool>,
}

fn parse_struct_component_attr(attr: &Attribute) -> Result<StructComponentAttrArgs, syn::Error> {
    if !matches!(attr.style, AttrStyle::Outer) {
        Err(syn::Error::new(attr.span(), "not an inner attribute"))
    } else if let Meta::List(list) = &attr.meta {
        let mut args = StructComponentAttrArgs::default();

        list.parse_nested_meta(|meta| {
            if meta.path.is_ident("tag") {
                let value = meta.value().and_then(|value| value.parse::<LitStr>())?;

                args.tag = Some(value.value());

                Ok(())
            } else if meta.path.is_ident("dynamic_tag") {
                let value = meta.value().and_then(|value| value.parse::<ExprArray>())?;

                args.dynamic_tag = Some(
                    value
                        .elems
                        .into_iter()
                        .filter_map(|elem| match &elem {
                            Expr::Path(path) => path.path.segments.last().map(|segment| {
                                (elem.clone(), segment.ident.to_string().to_lowercase())
                            }),
                            _ => None,
                        })
                        .collect(),
                );

                Ok(())
            } else if meta.path.is_ident("no_children") {
                let value = meta.value().and_then(|value| value.parse::<LitBool>())?;

                args.no_children = Some(value.value());

                Ok(())
            } else {
                Err(meta.error("unknown property"))
            }
        })?;

        Ok(args)
    } else {
        Err(syn::Error::new(attr.span(), "not a list"))
    }
}

#[proc_macro_attribute]
pub fn struct_component(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}

#[proc_macro_derive(StructComponent, attributes(struct_component))]
pub fn derive_struct_component(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let mut args = StructComponentAttrArgs::default();
    for attr in &derive_input.attrs {
        if attr.path().is_ident("struct_component") {
            match parse_struct_component_attr(attr) {
                Ok(result) => {
                    args = result;
                }
                Err(error) => {
                    return error.to_compile_error().into();
                }
            }
        }
    }

    if let Data::Struct(data_struct) = &derive_input.data {
        let ident = derive_input.ident.clone();

        let mut attributes: Vec<TokenStream> = vec![];
        // let mut attribute_checked: Option<TokenStream> = None;
        // let mut attribute_value: Option<TokenStream> = None;
        let mut listeners: Vec<TokenStream> = vec![];
        // let mut attributes_map: Option<TokenStream> = None;
        let mut dynamic_tag: Option<(Ident, Vec<(Expr, String)>)> = None;
        let mut node_ref: Option<TokenStream> = None;

        for field in &data_struct.fields {
            if let Some(ident) = &field.ident {
                if let Some(attr) = field
                    .attrs
                    .iter()
                    .find(|attr| attr.path().is_ident("struct_component"))
                {
                    match parse_struct_component_attr(attr) {
                        Ok(args) => {
                            if let Some(tags) = args.dynamic_tag {
                                dynamic_tag = Some((ident.clone(), tags));

                                continue;
                            }
                        }
                        Err(error) => {
                            return error.to_compile_error().into();
                        }
                    }
                }

                if ident == "attributes" {
                    // TODO: dynamic attributes

                    //     attributes_map = Some(quote! {
                    //         .chain(
                    //             self.attributes
                    //                 .into_iter()
                    //                 .flatten()
                    //                 .flat_map(|(key, value)| value.map(|value| (
                    //                     ::yew::virtual_dom::AttrValue::from(key),
                    //                     ::yew::virtual_dom::AttributeOrProperty::Attribute(AttrValue::from(value)),
                    //                 )),
                    //             ),
                    //         )
                    //     });

                    continue;
                }

                if ident == "node_ref" {
                    node_ref = Some(quote! {
                        .node_ref(self.node_ref)
                    });

                    continue;
                }

                if ident.to_string().starts_with("on")
                    && let Type::Path(path) = &field.ty
                {
                    let event = ident
                        .to_string()
                        .strip_prefix("on")
                        .expect("String should start with `on`.")
                        .parse::<TokenStream>()
                        .expect("String should parse as TokenStream.");

                    let first = path.path.segments.first();
                    let first_argument = first.and_then(|segment| match &segment.arguments {
                        PathArguments::None => None,
                        PathArguments::AngleBracketed(arguments) => {
                            arguments.args.first().and_then(|arg| match arg {
                                GenericArgument::Type(Type::Path(path)) => {
                                    path.path.segments.first()
                                }
                                _ => None,
                            })
                        }
                        PathArguments::Parenthesized(_) => None,
                    });

                    if first.is_some_and(|segment| segment.ident == "Callback") {
                        listeners.push(quote! {
                            .on(::leptos::tachys::html::event::#event, move |event| {
                                self.#ident.run(event);
                            })
                        });

                        continue;
                    } else if first.is_some_and(|segment| segment.ident == "Option")
                        && first_argument.is_some_and(|argument| argument.ident == "Callback")
                    {
                        listeners.push(quote! {
                            .on(::leptos::tachys::html::event::#event, move |event| {
                                if let Some(listener) = &self.#ident {
                                    listener.run(event);
                                }
                            })
                        });

                        continue;
                    }
                }

                match &field.ty {
                    Type::Path(path) => {
                        let first = path.path.segments.first();

                        attributes.push(
                            if first.is_some_and(|segment| segment.ident == "MaybeProp") {
                                quote! {
                                    .#ident(move || self.#ident.get())
                                }
                            } else {
                                quote! {
                                    .#ident(self.#ident)
                                }
                            },
                        );
                    }
                    _ => {
                        return syn::Error::new(field.ty.span(), "expected type path")
                            .to_compile_error()
                            .into();
                    }
                }
            }
        }

        let arguments = if args.no_children.unwrap_or(false) {
            quote! {
                self
            }
        } else {
            quote! {
                self, children: Option<::leptos::prelude::Children>
            }
        };

        let children = (!args.no_children.unwrap_or(false)).then(|| {
            quote! {
                .child(children.map(|children| children()))
            }
        });

        let tag_methods = quote! {
            // TODO: dynamic attributes

            #node_ref
                #(#attributes)*
                #(#listeners)*
                #children
                .into_any()
        };

        if let Some((tag_ident, tags)) = dynamic_tag {
            let exprs = tags.iter().map(|(expr, _)| expr).collect::<Vec<_>>();
            let tags = tags
                .iter()
                .map(|(_, tag)| {
                    let tag = format!("::leptos::html::{tag}()")
                        .parse::<TokenStream>()
                        .expect("String should parse as TokenStream.");

                    quote! {
                        #tag
                        #tag_methods
                    }
                })
                .collect::<Vec<_>>();

            quote! {
                impl #ident {
                    pub fn render(#arguments) -> ::leptos::tachys::view::any_view::AnyView {
                        match self.#tag_ident {
                            #(#exprs => #tags,)*
                        }
                    }
                }
            }
            .into()
        } else if let Some(tag) = args.tag {
            let tag = format!("::leptos::html::{tag}()")
                .parse::<TokenStream>()
                .expect("String should parse as TokenStream.");

            quote! {
                impl #ident {
                    pub fn render(#arguments) -> ::leptos::tachys::view::any_view::AnyView {
                        #tag
                        #tag_methods
                    }
                }
            }
            .into()
        } else {
            syn::Error::new(derive_input.span(), "`#[struct_component(tag = \"\")] or #[struct_component(dynamic_tag = true)]` is required")
                    .to_compile_error()
                    .into()
        }
    } else {
        syn::Error::new(derive_input.span(), "expected struct")
            .to_compile_error()
            .into()
    }
}
