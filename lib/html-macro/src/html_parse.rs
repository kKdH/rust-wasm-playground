use syn::Ident;
use syn::parse::{Parse, ParseStream};
use syn::parse::Result;
use syn::Token;

use vdom::{VRef, VTree};

use crate::html::Html;

impl Parse for Html {

    fn parse(input: ParseStream) -> Result<Self> {

        let mut parse_stack: Vec<VRef> = Vec::new();
        let mut tree = VTree::new();

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(Token![<]) {
                if input.peek2(Token![/]) {
                    input.parse::<Token![<]>()?;
                    input.parse::<Token![/]>()?;
                    let name = input.parse::<Ident>()?;
                    input.parse::<Token![>]>()?;
                    match parse_stack.pop() {
                        None => {
                            panic!("Closing tag '{:?}' does not have a corresponding element!", name.to_string())
                        }
                        Some(node) => {
                            let node = tree
                                .get_node(&node)
                                .expect(format!("Node with vref: {:?}", node).as_str());

                            if node.kind != name.to_string() {
                                panic!("Expected closing tag for '{:?}' got '{:?}'!", node.kind, name.to_string());
                            }
                        }
                    }
                }
                else {
                    input.parse::<Token![<]>()?;
                    let node = tree.create_random_node();
                    match parse_stack.last() {
                        None => {
                            if tree.has_root() {
                                panic!("Root node already set!")
                            }
                            else {
                                tree.set_root(&node);
                            }
                        }
                        Some(parent) => {
                            tree.append_child(parent, &node)
                        }
                    }
                    parse_stack.push(node);
                }
            } else if lookahead.peek(Ident) {
                if let Some(node) = parse_stack.last() {
                    let ident = input.parse::<Ident>()?;
                    let kind = ident.to_string();
                    tree.update_node(node, Box::new(|node| {
                        node.kind = kind
                    }));
                }
            } else if lookahead.peek(Token![>]) {
                input.parse::<Token![>]>()?;
            } else {
                panic!("unknown token")
            }
        }

        Ok(Self::new(tree))
    }
}
