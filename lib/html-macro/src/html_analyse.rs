use std::fmt::{Debug, Formatter};
use crate::html::{Html, HtmlAttribute, HtmlElement};
use crate::html_parse::{HtmlToken};
use crate::HtmlTokenStream;

pub(crate) type AnalyseResult = Result<Html, AnalyseError>;

#[derive(Debug)]
pub enum AnalyseError {
    Failure { message: &'static str },
    UnexpectedToken { message: &'static str, token: HtmlToken },
    UnexpectedEnd,
}

pub fn analyse_html(input: HtmlTokenStream) -> AnalyseResult {
    let mut context = AnalyseContext { stack: Vec::new(), attribute: None, html: None };
    let mut behavior: Behavior = Behavior::of(initial);
    let mut position: usize = 0;

    let result = loop {
        match input.get(position) {
            Some(token) => {
                let next_behavior = behavior.handle(&mut context, token);
                behavior = match next_behavior {
                    Behavior::Custom(_) |
                    Behavior::Same => next_behavior,
                    Behavior::Done => {
                        match context.html {
                            None => break Err(AnalyseError::Failure { message: "No result" }),
                            Some(html) => break Ok(html),
                        }
                    },
                    Behavior::Failure(error) => break Err(error),
                };
            }
            None => {
                break Err(AnalyseError::UnexpectedEnd)
            }
        }
        position += 1;
    };

    result
}

struct AnalyseContext {
    stack: Vec<HtmlElement>,
    attribute: Option<HtmlAttribute>,
    html: Option<Html>
}

type BehaviorFn = fn(&mut AnalyseContext, &HtmlToken) -> Behavior;

enum Behavior {
    Custom(BehaviorFn),
    Same,
    Done,
    Failure(AnalyseError)
}

impl Behavior {

    fn of(behavior_fn: BehaviorFn) -> Behavior {
        Behavior::Custom(behavior_fn)
    }

    fn same() -> Behavior {
        Behavior::Same
    }

    fn done() -> Behavior {
        Behavior::Done
    }

    fn fail(message: &'static str) -> Behavior {
        Behavior::Failure(AnalyseError::Failure { message })
    }

    fn unexpected(message: &'static str, token: &HtmlToken) -> Behavior {
        Behavior::Failure(AnalyseError::UnexpectedToken { message, token: (*token).clone() })
    }

    fn handle(self, context: &mut AnalyseContext, token: &HtmlToken) -> Self {
        match self {
            Behavior::Same => panic!("'Same' can not be the first behavior!"),
            Behavior::Done => Behavior::Done,
            Behavior::Failure(_) => panic!("A1"), //self,
            Behavior::Custom(behavior_fn) => {
                let next_behavior = behavior_fn(context, token);
                match next_behavior {
                    Behavior::Custom(_) => next_behavior,
                    Behavior::Same => self,
                    Behavior::Done => Behavior::Done,
                    Behavior::Failure(_) => next_behavior,
                }
            }
        }
    }
}

impl Debug for Behavior {

    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = match self {
            Behavior::Custom(_) => {
                formatter.debug_struct("Custom")
            }
            Behavior::Same => {
                formatter.debug_struct("Same")
            }
            Behavior::Done => {
                formatter.debug_struct("Done")
            }
            Behavior::Failure(_) => {
                formatter.debug_struct("Failure")
            }
        };
        result.finish()
    }
}

fn initial(_: &mut AnalyseContext, token: &HtmlToken) -> Behavior {
    match token {
        HtmlToken::LessThan => Behavior::of(analyse_element_start),
        HtmlToken::GreaterThan => Behavior::fail("Unexpected >"),
        HtmlToken::Slash => Behavior::fail("Unexpected /"),
        HtmlToken::Eq => Behavior::fail("Unexpected ="),
        HtmlToken::ElementStart { .. } => Behavior::fail("Unexpected element start"),
        HtmlToken::ElementEnd { .. } => Behavior::fail("Unexpected end of element"),
        HtmlToken::AttributeName { .. } => Behavior::fail("Unexpected attribute name"),
        HtmlToken::AttributeValue { .. } => Behavior::fail("Unexpected attribute value"),
        HtmlToken::Text { .. } => Behavior::fail("Unexpected text"),
        HtmlToken::EOF => Behavior::unexpected("Got unexpected token in initial behavior!", token)
    }
}

fn analyse_element_start(context: &mut AnalyseContext, token: &HtmlToken) -> Behavior {
    match token {
        HtmlToken::LessThan => Behavior::fail("Unexpected <"),
        HtmlToken::GreaterThan => Behavior::fail("Unexpected >"),
        HtmlToken::Slash => Behavior::of(analyse_element_end),
        HtmlToken::Eq => Behavior::fail("Unexpected ="),
        HtmlToken::ElementStart { ident } => {
            context.stack.push(HtmlElement::new(ident.to_string(), Vec::new(), Vec::new(), None));
            Behavior::of(analyse_element_attributes)
        },
        HtmlToken::ElementEnd { .. } => Behavior::fail("Unexpected <"),
        HtmlToken::AttributeName { .. } => Behavior::fail("Unexpected attribute name"),
        HtmlToken::AttributeValue { .. } => Behavior::fail("Unexpected attribute value"),
        HtmlToken::Text { .. } => Behavior::fail("Unexpected text"),
        HtmlToken::EOF => Behavior::unexpected("Got unexpected token while analysing the start of an element!", token)
    }
}

fn analyse_element_end(context: &mut AnalyseContext, token: &HtmlToken) -> Behavior {
    match token {
        HtmlToken::LessThan => Behavior::of(analyse_element_start),
        HtmlToken::GreaterThan => {
            match context.stack.pop() {
                None => Behavior::fail("No element"),
                Some(element) => {
                    match context.stack.last_mut() {
                        Some(parent) => {
                            parent.add_child(element);
                            Behavior::of(initial)
                        }
                        None => {
                            // Because their is no parent it must be the root element.
                            // Therefore push it back onto the stack.
                            context.stack.push(element);
                            Behavior::of(analyse_end)
                        }
                    }

                }
            }
        },
        HtmlToken::Slash => Behavior::same(),
        HtmlToken::Eq => Behavior::fail("Unexpected ="),
        HtmlToken::ElementStart { .. } => Behavior::fail("Unexpected element start"),
        HtmlToken::ElementEnd { .. } => Behavior::same(),
        HtmlToken::AttributeName { .. } => Behavior::fail("Unexpected attribute name"),
        HtmlToken::AttributeValue { .. } => Behavior::fail("Unexpected attribute value"),
        HtmlToken::Text { literal } => {
            match context.stack.last_mut() {
                None => Behavior::fail("Unexpected text"),
                Some(element) => {
                    element.set_text(Some(literal.value()));
                    Behavior::Same
                }
            }
        },
        HtmlToken::EOF => Behavior::unexpected("Got unexpected token while analysing the end of an element!", token)
    }
}

fn analyse_element_attributes(context: &mut AnalyseContext, token: &HtmlToken) -> Behavior {
    match token {
        HtmlToken::LessThan => Behavior::fail("Unexpected <"),
        HtmlToken::GreaterThan => Behavior::of(analyse_element_end),
        HtmlToken::Slash => Behavior::of(analyse_element_end),
        HtmlToken::Eq => Behavior::same(),
        HtmlToken::ElementStart { .. } => Behavior::fail("Unexpected element start"),
        HtmlToken::ElementEnd { .. } => Behavior::fail("Unexpected <"),
        HtmlToken::AttributeName { ident } => {
            context.attribute = Some(HtmlAttribute::new(ident.to_string(), None));
            Behavior::same()
        },
        HtmlToken::AttributeValue { literal } => {
            match (context.stack.last_mut(), context.attribute.take()) {
                (Some(element), Some(mut attribute)) => {
                    attribute.value = Some(literal.value());
                    element.add_attribute(attribute);
                    Behavior::same()
                }
                (_, None) => Behavior::fail("No attribute"),
                (None, _) => Behavior::fail("No element"),
            }
        },
        HtmlToken::Text { .. } => Behavior::fail("Unexpected text"),
        HtmlToken::EOF => Behavior::unexpected("Got unexpected token while analysing the attributes of an element!", token)
    }
}

fn analyse_end(context: &mut AnalyseContext, token: &HtmlToken) -> Behavior {
    match token {
        HtmlToken::LessThan => Behavior::fail("Unexpected <"),
        HtmlToken::GreaterThan => Behavior::fail("Unexpected >"),
        HtmlToken::Slash => Behavior::fail("Unexpected /"),
        HtmlToken::Eq => Behavior::fail("Unexpected ="),
        HtmlToken::ElementStart { .. } => Behavior::fail("Unexpected element start"),
        HtmlToken::ElementEnd { .. } => Behavior::fail("Unexpected <"),
        HtmlToken::AttributeName { .. } => Behavior::fail("Unexpected attribute start"),
        HtmlToken::AttributeValue { .. } => Behavior::fail("Unexpected attribute value"),
        HtmlToken::Text { .. } => Behavior::fail("Unexpected text"),
        HtmlToken::EOF => {
            match context.stack.pop() {
                None => {
                    Behavior::fail("No element left on stack!")
                }
                Some(element) => {
                    context.html = Some(Html::new(element));
                    Behavior::done()
                }
            }
        },
    }
}

#[cfg(test)]
mod test {
    use speculoos::prelude::*;
    use proc_macro2::{Ident, Span};
    use syn::LitStr;
    use crate::html::{Html, HtmlAttribute, HtmlElement};
    use crate::html_analyse::{analyse_html, AnalyseError};
    use crate::html_parse::HtmlToken;
    use crate::HtmlTokenStream;

    #[test]
    fn test_analyse_element() {

        let input = HtmlTokenStream::new(vec![
            HtmlToken::LessThan,
            HtmlToken::ElementStart { ident: Ident::new("div", Span::call_site()) },
            HtmlToken::GreaterThan,
            HtmlToken::LessThan,
            HtmlToken::Slash,
            HtmlToken::ElementEnd { ident: Some(Ident::new("div", Span::call_site())) },
            HtmlToken::GreaterThan,
            HtmlToken::EOF
        ]);

        let html: Result<Html, AnalyseError> = analyse_html(input);

        assert_that(&html).is_ok();
        assert_that(html.ok().unwrap().root())
            .is_equal_to(HtmlElement::new(String::from("div"), Vec::new(), Vec::new(), None))
    }

    #[test]
    fn test_analyse_element_with_attributes() {

        let input = HtmlTokenStream::new(vec![
            HtmlToken::LessThan,
            HtmlToken::ElementStart { ident: Ident::new("div", Span::call_site()) },
            HtmlToken::AttributeName { ident: Ident::new("id", Span::call_site()) },
            HtmlToken::Eq,
            HtmlToken::AttributeValue { literal: LitStr::new("container", Span::call_site()) },
            HtmlToken::GreaterThan,
            HtmlToken::LessThan,
            HtmlToken::Slash,
            HtmlToken::ElementEnd { ident: Some(Ident::new("div", Span::call_site())) },
            HtmlToken::GreaterThan,
            HtmlToken::EOF
        ]);

        let html: Result<Html, AnalyseError> = analyse_html(input);

        assert_that(&html).is_ok();
        assert_that(html.ok().unwrap().root())
            .is_equal_to(HtmlElement::new(
                String::from("div"),
                vec![
                    HtmlAttribute::new(String::from("id"), Some(String::from("container")))
                ],
                Vec::new(),
                None
            ))
    }

    #[test]
    fn test_analyse_void_element() {

        let input = HtmlTokenStream::new(vec![
            HtmlToken::LessThan,
            HtmlToken::ElementStart { ident: Ident::new("img", Span::call_site()) },
            HtmlToken::Slash,
            HtmlToken::ElementEnd { ident: None },
            HtmlToken::GreaterThan,
            HtmlToken::EOF
        ]);

        let html: Result<Html, AnalyseError> = analyse_html(input);

        assert_that(&html).is_ok();
        assert_that(html.ok().unwrap().root())
            .is_equal_to(HtmlElement::new(String::from("img"), Vec::new(), Vec::new(), None))
    }

    #[test]
    fn test_analyse_element_with_children() {

        let input = HtmlTokenStream::new(vec![
            HtmlToken::LessThan,
            HtmlToken::ElementStart { ident: Ident::new("div", Span::call_site()) },
            HtmlToken::GreaterThan,
            HtmlToken::LessThan,
            HtmlToken::ElementStart { ident: Ident::new("p", Span::call_site()) },
            HtmlToken::GreaterThan,
            HtmlToken::LessThan,
            HtmlToken::Slash,
            HtmlToken::ElementEnd { ident: Some(Ident::new("p", Span::call_site())) },
            HtmlToken::GreaterThan,
            HtmlToken::LessThan,
            HtmlToken::Slash,
            HtmlToken::ElementEnd { ident: Some(Ident::new("div", Span::call_site())) },
            HtmlToken::GreaterThan,
            HtmlToken::EOF
        ]);

        let html: Result<Html, AnalyseError> = analyse_html(input);

        assert_that(&html).is_ok();
        assert_that(html.ok().unwrap().root())
            .is_equal_to(HtmlElement::new(
                String::from("div"),
                Vec::new(),
                vec![
                    HtmlElement::new(
                        String::from("p"),
                        Vec::new(),
                        Vec::new(),
                        None
                    ),
                ],
                None
            ));
    }

    #[test]
    fn test_analyse_element_with_text_content() {

        let input = HtmlTokenStream::new(vec![
            HtmlToken::LessThan,
            HtmlToken::ElementStart { ident: Ident::new("div", Span::call_site()) },
            HtmlToken::GreaterThan,
            HtmlToken::Text { literal: LitStr::new("hello world", Span::call_site()) },
            HtmlToken::LessThan,
            HtmlToken::Slash,
            HtmlToken::ElementEnd { ident: Some(Ident::new("div", Span::call_site())) },
            HtmlToken::GreaterThan,
            HtmlToken::EOF
        ]);

        let html: Result<Html, AnalyseError> = analyse_html(input);

        assert_that(&html).is_ok();
        assert_that(html.ok().unwrap().root())
            .is_equal_to(HtmlElement::new(
                String::from("div"),
                Vec::new(),
                Vec::new(),
                Some(String::from("hello world"))
            ))
    }
}
