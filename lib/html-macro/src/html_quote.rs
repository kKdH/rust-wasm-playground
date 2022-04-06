use std::collections::VecDeque;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::LitStr;
use vdom::{VNode, VRef};
use crate::html::Html;

impl ToTokens for Html {

    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let root_node = self.tree.get_root().expect("No node");
        let root_node_uuid_string = LitStr::new(String::from(root_node).as_str(), Span::call_site());
        let mut node_queue: VecDeque<VRef> = VecDeque::new();
        let mut quotes: Vec<TokenStream> = Vec::new();

        node_queue.push_back(root_node);

        while !node_queue.is_empty() {
            if let Some(node_ref) = node_queue.pop_front() {
                let node: VNode = self.tree.get_node(&node_ref).expect("No node");
                let name = LitStr::new(&node.kind, Span::call_site());
                let node_ref_uuid_string = LitStr::new(String::from(node.id).as_str(), Span::call_site());

                quotes.push(quote! {
                    let parent = {
                        let node_ref = <vdom::VRef>::from_string(String::from(#node_ref_uuid_string)).ok().unwrap();
                        tree.create_node(&node_ref);
                        tree.update_node(&node_ref, Box::new(|node| {
                            node.kind = String::from(#name);
                        }));
                        node_ref
                    };
                });

                self.tree
                    .children(&node_ref)
                    .iter()
                    .for_each(|child| {
                        let uuid_string = LitStr::new(String::from(child.id).as_str(), Span::call_site());
                        node_queue.push_back(child.id);
                        quotes.push(quote! {
                            {
                                let child_ref = <vdom::VRef>::from_string(String::from(#uuid_string)).ok().unwrap();
                                tree.append_child(&parent, &child_ref);
                            }
                        });
                    })
            }
        }

        tokens.extend(quote! {
            {
                let mut tree = <vdom::VTree>::new();
                let root_node_ref = <vdom::VRef>::from_string(String::from(#root_node_uuid_string)).ok().unwrap();
                tree.set_root(&root_node_ref);
                #(#quotes)*
                tree
            }
        });
    }
}
