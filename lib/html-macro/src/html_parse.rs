use proc_macro::TokenStream;
use std::fmt::{Debug, Formatter};

use syn::{Ident, LitStr};
use syn::parse::{Parse, ParseStream};
use syn::parse::Result;
use syn::Token;

use vdom::{VRef, VTree};

use crate::html::Html;

pub fn parse_html(input: TokenStream) -> HtmlTokenStream {
    ::syn::parse::<HtmlTokenStream>(input).unwrap()
}

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

#[derive(Debug)]
pub struct HtmlTokenStream {
    tokens: Vec<HtmlToken>
}

impl HtmlTokenStream {

    pub fn new(tokens: Vec<HtmlToken>) -> HtmlTokenStream {
        HtmlTokenStream { tokens }
    }

    pub fn get(& self, position: usize) -> Option<&HtmlToken> {
        self.tokens.get(position)
    }
}

impl Parse for HtmlTokenStream {

    fn parse(input: ParseStream) -> Result<Self> {

        let mut tokens: Vec<HtmlToken> = Vec::new();
        while let Ok(mut token) = HtmlToken::parse(input, &tokens) {
            tokens.append(&mut token);
        }

        tokens.push(HtmlToken::EOF);

        Ok(HtmlTokenStream {
            tokens
        })
    }
}

#[derive(Clone)]
pub enum HtmlToken {
    LessThan,
    GreaterThan,
    Slash,
    Eq,
    ElementStart { ident: Ident },
    ElementEnd { ident: Option<Ident> },
    AttributeName { ident: Ident },
    AttributeValue { literal: LitStr },
    Text { literal: LitStr },
    EOF
}

impl HtmlToken {

    fn parse(input: ParseStream, tokens: &Vec<HtmlToken>) -> Result<Vec<HtmlToken>> {
        if let Ok(_) = input.parse::<Token![<]>() {
            Ok(vec![HtmlToken::LessThan])
        }
        else if let Ok(_) = input.parse::<Token![>]>() {
            match tokens.last() {
                Some(HtmlToken::Slash) => {
                    Ok(vec![
                        HtmlToken::ElementEnd { ident: None },
                        HtmlToken::GreaterThan
                    ])
                }
                Some(HtmlToken::ElementStart { .. }) |
                Some(HtmlToken::ElementEnd { .. }) |
                Some(HtmlToken::AttributeName { .. }) |
                Some(HtmlToken::AttributeValue { .. }) => {
                    Ok(vec![HtmlToken::GreaterThan])
                }
                _ => {
                    Err(input.error("Invalid closing character '>'!"))
                }
            }
        }
        else if let Ok(_) = input.parse::<Token![/]>() {
            Ok(vec![HtmlToken::Slash])
        }
        else if let Ok(_) = input.parse::<Token![=]>() {
            Ok(vec![HtmlToken::Eq])
        }
        else if let Ok(ident) = input.parse::<Ident>() {
            match tokens.last() {
                Some(HtmlToken::LessThan) => {
                    Ok(vec![HtmlToken::ElementStart { ident }])
                }
                Some(HtmlToken::Slash) => {
                    Ok(vec![HtmlToken::ElementEnd { ident: Some(ident) }])
                }
                Some(HtmlToken::ElementStart { .. }) | Some(HtmlToken::AttributeValue { .. }) => {
                    Ok(vec![HtmlToken::AttributeName { ident }])
                }
                _ => {
                    Err(input.error("No element start token '<' found!"))
                }
            }
        }
        else if let Ok(literal) = input.parse::<LitStr>() {
            match tokens.last() {
                Some(HtmlToken::Eq) => {
                    Ok(vec![HtmlToken::AttributeValue { literal }])
                }
                Some(HtmlToken::GreaterThan) => {
                    Ok(vec![HtmlToken::Text { literal }])
                }
                _ => {
                    Err(input.error("No attribute assignment token '=' found!"))
                }
            }
        }
        else {
            Err(input.error("Unknown"))
        }
    }
}

impl Debug for HtmlToken {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HtmlToken::LessThan => {
                formatter.debug_struct("HtmlToken::LessThan")
                    .finish()
            }
            HtmlToken::GreaterThan => {
                formatter.debug_struct("HtmlToken::GreaterThan")
                    .finish()
            }
            HtmlToken::Slash => {
                formatter.debug_struct("HtmlToken::Slash")
                    .finish()
            }
            HtmlToken::Eq => {
                formatter.debug_struct("HtmlToken::Eq")
                    .finish()
            }
            HtmlToken::ElementStart { ident } => {
                formatter.debug_struct("HtmlToken::ElementStart")
                    .field("ident", ident)
                    .finish()
            }
            HtmlToken::ElementEnd { ident } => {
                formatter.debug_struct("HtmlToken::ElementEnd")
                    .field("ident", ident)
                    .finish()
            }
            HtmlToken::AttributeName { ident } => {
                formatter.debug_struct("HtmlToken::AttributeName")
                    .field("ident", ident)
                    .finish()
            }
            HtmlToken::AttributeValue { .. } => {
                formatter.debug_struct("HtmlToken::AttributeValue")
                    .finish()
            }
            HtmlToken::Text { .. } => {
                formatter.debug_struct("HtmlToken::Text")
                    .finish()
            }
            HtmlToken::EOF => {
                formatter.debug_struct("HtmlToken::End")
                    .finish()
            }
        }
    }
}

impl PartialEq for HtmlToken {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (HtmlToken::LessThan, HtmlToken::LessThan) => true,
            (HtmlToken::GreaterThan, HtmlToken::GreaterThan) => true,
            (HtmlToken::Eq, HtmlToken::Eq) => true,
            (HtmlToken::Slash, HtmlToken::Slash) => true,
            (HtmlToken::ElementStart { ident: a }, HtmlToken::ElementStart { ident: b}) => a == b,
            (HtmlToken::ElementEnd { ident: a }, HtmlToken::ElementEnd { ident: b}) => a == b,
            (HtmlToken::AttributeName { ident: a}, HtmlToken::AttributeName { ident: b }) => a == b,
            (HtmlToken::AttributeValue { literal: a}, HtmlToken::AttributeValue { literal: b }) => a.value() == b.value(),
            (HtmlToken::Text { literal: a}, HtmlToken::Text { literal: b }) => a.value() == b.value(),
            (HtmlToken::EOF, HtmlToken::EOF) => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod test_html_token_parsing {
    use proc_macro2::{Ident, Span};
    use speculoos::prelude::*;
    use syn::{LitStr, parse_quote};

    use crate::html_parse::{HtmlTokenStream, HtmlToken};

    #[test]
    fn test_parse_html_element_start() {

        let html: HtmlTokenStream = parse_quote! {
            <div>
        };

        assert_that(&html.tokens)
            .is_equal_to(&vec![
                HtmlToken::LessThan,
                HtmlToken::ElementStart {ident: Ident::new("div", Span::call_site())},
                HtmlToken::GreaterThan,
                HtmlToken::EOF
            ]);
    }

    #[test]
    fn test_parse_html_element_end() {
        {
            let html: HtmlTokenStream = parse_quote! {
                <div></div>
            };

            assert_that(&html.tokens)
                .is_equal_to(&vec![
                    HtmlToken::LessThan,
                    HtmlToken::ElementStart { ident: Ident::new("div", Span::call_site()) },
                    HtmlToken::GreaterThan,
                    HtmlToken::LessThan,
                    HtmlToken::Slash,
                    HtmlToken::ElementEnd { ident: Some(Ident::new("div", Span::call_site())) },
                    HtmlToken::GreaterThan,
                    HtmlToken::EOF
                ]);
        }

        {
            let html: HtmlTokenStream = parse_quote! {
                <div/>
            };

            assert_that(&html.tokens)
                .is_equal_to(&vec![
                    HtmlToken::LessThan,
                    HtmlToken::ElementStart { ident: Ident::new("div", Span::call_site()) },
                    HtmlToken::Slash,
                    HtmlToken::ElementEnd { ident: None },
                    HtmlToken::GreaterThan,
                    HtmlToken::EOF
                ]);
        }
    }

    #[test]
    fn test_parse_html_element_attributes() {

        let html: HtmlTokenStream = parse_quote! {
            <div id="myElement" class="foobar">
        };

        assert_that(&html.tokens)
            .is_equal_to(&vec![
                HtmlToken::LessThan,
                HtmlToken::ElementStart { ident: Ident::new("div", Span::call_site()) },
                HtmlToken::AttributeName { ident: Ident::new("id", Span::call_site()) },
                HtmlToken::Eq,
                HtmlToken::AttributeValue { literal: LitStr::new("myElement", Span::call_site()) },
                HtmlToken::AttributeName { ident: Ident::new("class", Span::call_site()) },
                HtmlToken::Eq,
                HtmlToken::AttributeValue { literal: LitStr::new("foobar", Span::call_site()) },
                HtmlToken::GreaterThan,
                HtmlToken::EOF
            ]);
    }

    #[test]
    fn test_parse_html_element_text() {

        let html: HtmlTokenStream = parse_quote! {
            <p>"Hello World"</p>
        };

        assert_that(&html.tokens)
            .is_equal_to(&vec![
                HtmlToken::LessThan,
                HtmlToken::ElementStart { ident: Ident::new("p", Span::call_site()) },
                HtmlToken::GreaterThan,
                HtmlToken::Text { literal: LitStr::new("Hello World", Span::call_site()) },
                HtmlToken::LessThan,
                HtmlToken::Slash,
                HtmlToken::ElementEnd { ident: Some(Ident::new("p", Span::call_site())) },
                HtmlToken::GreaterThan,
                HtmlToken::EOF
            ]);
    }

    #[test]
    fn test_parse_nested_html_elements() {

        let html: HtmlTokenStream = parse_quote! {
            <div>
                <p>"Hello World"</p>
                <a>
                    <b/>
                </a>
            </div>
        };

        assert_that(&html.tokens)
            .is_equal_to(&vec![
                HtmlToken::LessThan,
                HtmlToken::ElementStart { ident: Ident::new("div", Span::call_site()) },
                HtmlToken::GreaterThan,
                HtmlToken::LessThan,
                HtmlToken::ElementStart { ident: Ident::new("p", Span::call_site()) },
                HtmlToken::GreaterThan,
                HtmlToken::Text { literal: LitStr::new("Hello World", Span::call_site()) },
                HtmlToken::LessThan,
                HtmlToken::Slash,
                HtmlToken::ElementEnd { ident: Some(Ident::new("p", Span::call_site())) },
                HtmlToken::GreaterThan,
                HtmlToken::LessThan,
                HtmlToken::ElementStart { ident: Ident::new("a", Span::call_site()) },
                HtmlToken::GreaterThan,
                HtmlToken::LessThan,
                HtmlToken::ElementStart { ident: Ident::new("b", Span::call_site()) },
                HtmlToken::Slash,
                HtmlToken::ElementEnd { ident: None },
                HtmlToken::GreaterThan,
                HtmlToken::LessThan,
                HtmlToken::Slash,
                HtmlToken::ElementEnd { ident: Some(Ident::new("a", Span::call_site())) },
                HtmlToken::GreaterThan,
                HtmlToken::LessThan,
                HtmlToken::Slash,
                HtmlToken::ElementEnd { ident: Some(Ident::new("div", Span::call_site())) },
                HtmlToken::GreaterThan,
                HtmlToken::EOF
            ]);
    }
}
