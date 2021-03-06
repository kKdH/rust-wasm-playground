use std::collections::VecDeque;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::LitStr;
use uuid::Uuid;

use crate::html::HtmlElement;
use crate::Html;

impl ToTokens for Html {

    fn to_tokens(&self, tokens: &mut TokenStream) {

        let root_element = self.root();
        let mut element_queue: VecDeque<(Option<Uuid>, &HtmlElement)> = VecDeque::new();
        let mut quotes: Vec<TokenStream> = Vec::new();

        element_queue.push_back((None, root_element));

        while !element_queue.is_empty() {
            if let Some((parent, element)) = element_queue.pop_front() {
                let node_uuid = Uuid::new_v4();
                let node_ref_uuid_literal = LitStr::new(node_uuid.to_string().as_str(), Span::call_site());
                let node_name_literal = LitStr::new(element.get_name().as_str(), Span::call_site());

                let attributes = element.attributes().iter().fold(TokenStream::new(), |mut result, attr| {
                    let name_literal = LitStr::new(attr.name.as_str(), Span::call_site());
                    let value_literal = attr.value.clone().map_or_else(|| LitStr::new("", Span::call_site()),|value| {
                        LitStr::new(value.as_str(), Span::call_site())
                    });
                    result.extend(quote! {
                        (String::from(#name_literal), String::from(#value_literal)),
                    });
                    result
                });

                let text_content = match element.get_text() {
                    None => {
                        quote! { core::option::Option::None }
                    }
                    Some(text) => {
                        let text_literal = LitStr::new(text.as_str(), Span::call_site());
                        quote! {
                            core::option::Option::Some(String::from(#text_literal))
                        }
                    }
                };

                quotes.push(quote! {
                    {
                        let node_ref = <vdom::VRef>::from_string(String::from(#node_ref_uuid_literal)).ok().unwrap();
                        tree.create_node(&node_ref);
                        tree.update_node(&node_ref, Box::new(|node| {
                            node.item = core::option::Option::Some(vdom::VItem::Element {
                                name: String::from(#node_name_literal),
                                attributes: vec![#attributes],
                                text: #text_content,
                            });
                        }));
                    };
                });

                match parent {
                    None => { // root element
                        quotes.push(quote! {
                            let root_node_ref = <vdom::VRef>::from_string(String::from(#node_ref_uuid_literal)).ok().unwrap();
                            tree.set_root(&root_node_ref);
                        });
                    }
                    Some(parent) => {
                        let parent_ref_uuid_literal = LitStr::new(parent.to_string().as_str(), Span::call_site());
                        quotes.push(quote! {
                            {
                                let parent_node_ref = <vdom::VRef>::from_string(String::from(#parent_ref_uuid_literal)).ok().unwrap();
                                let child_node_ref = <vdom::VRef>::from_string(String::from(#node_ref_uuid_literal)).ok().unwrap();
                                tree.append_child(&parent_node_ref, &child_node_ref);
                            }
                        });
                    }
                }

                element.children().iter().for_each(|child| {
                    element_queue.push_back((Some(node_uuid), child))
                });
            }
        }

        tokens.extend(quote! {
            {
                let mut tree = <vdom::VTree>::new();
                #(#quotes)*
                tree
            }
        });
    }
}
