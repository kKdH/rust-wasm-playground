use crate::html::{Html2, HtmlAttribute, HtmlElement};
use crate::html_parse::{HtmlToken, HtmlTokenStreamCursor};
use crate::HtmlTokenStream;

pub fn analyse(input: HtmlTokenStream) -> Html2 {
    let mut cursor: HtmlTokenStreamCursor = input.cursor();
    let mut context = AnalyseContext { stack: Vec::new(), attribute: None, html: Html2::new() };
    let mut behavior: Behavior = Behavior::of(initial);

    while let Some(token) = input.get(&mut cursor) {
        behavior = behavior.handle(&mut context, token)
    }

    context.html
}

struct AnalyseContext {
    stack: Vec<HtmlElement>,
    attribute: Option<HtmlAttribute>,
    html: Html2
}

type BehaviorFn = fn(&mut AnalyseContext, &HtmlToken) -> Behavior;

enum Behavior {
    Custom(BehaviorFn),
    Same,
    Failure(&'static str)
}

impl Behavior {

    fn of(behavior_fn: BehaviorFn) -> Behavior {
        Behavior::Custom(behavior_fn)
    }

    fn same() -> Behavior {
        Behavior::Same
    }

    fn fail(message: &'static str) -> Behavior {
        Behavior::Failure(message)
    }

    fn handle(self, context: &mut AnalyseContext, token: &HtmlToken) -> Self {
        match self {
            Behavior::Same => panic!("'Same' can not be the first behavior!"),
            Behavior::Failure(message) => panic!("{}", message),
            Behavior::Custom(behavior_fn) => {
                let next_behavior = behavior_fn(context, token);
                match next_behavior {
                    Behavior::Custom(_) => next_behavior,
                    Behavior::Same => self,
                    Behavior::Failure(message) => panic!("{}", message),
                }
            }
        }
    }
}

fn initial(_: &mut AnalyseContext, token: &HtmlToken) -> Behavior {
    println!("Behavior: initial, Token: {:?}", token);
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
        HtmlToken::End => Behavior::fail("Unexpected end")
    }
}

fn analyse_element_start(context: &mut AnalyseContext, token: &HtmlToken) -> Behavior {
    println!("Behavior: analyse_element_start, Token: {:?}", token);
    match token {
        HtmlToken::LessThan => Behavior::fail("Unexpected <"),
        HtmlToken::GreaterThan => Behavior::fail("Unexpected >"),
        HtmlToken::Slash => Behavior::of(analyse_element_end),
        HtmlToken::Eq => Behavior::fail("Unexpected ="),
        HtmlToken::ElementStart { ident } => {
            context.stack.push(HtmlElement::new(ident.to_string(), Vec::new()));
            Behavior::of(analyse_element_attributes)
        },
        HtmlToken::ElementEnd { .. } => Behavior::fail("Unexpected <"),
        HtmlToken::AttributeName { .. } => Behavior::fail("Unexpected attribute name"),
        HtmlToken::AttributeValue { .. } => Behavior::fail("Unexpected attribute value"),
        HtmlToken::Text { .. } => Behavior::fail("Unexpected text"),
        HtmlToken::End => Behavior::fail("Unexpected end"),
    }
}

fn analyse_element_end(context: &mut AnalyseContext, token: &HtmlToken) -> Behavior {
    println!("Behavior: analyse_element_end, Token: {:?}", token);
    match token {
        HtmlToken::LessThan => Behavior::of(analyse_element_start),
        HtmlToken::GreaterThan => {
            context.html.nodes.push(context.stack.pop().unwrap());
            Behavior::of(initial)
        },
        HtmlToken::Slash => Behavior::same(),
        HtmlToken::Eq => Behavior::fail("Unexpected ="),
        HtmlToken::ElementStart { .. } => Behavior::fail("Unexpected element start"),
        HtmlToken::ElementEnd { .. } => Behavior::same(),
        HtmlToken::AttributeName { .. } => Behavior::fail("Unexpected attribute name"),
        HtmlToken::AttributeValue { .. } => Behavior::fail("Unexpected attribute value"),
        HtmlToken::Text { .. } => Behavior::fail("Unexpected text"),
        HtmlToken::End => Behavior::fail("Unexpected end"),
    }
}

fn analyse_element_attributes(context: &mut AnalyseContext, token: &HtmlToken) -> Behavior {
    println!("Behavior: analyse_element_attributes, Token: {:?}", token);
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
        HtmlToken::End => Behavior::fail("Unexpected end"),
    }
}

#[cfg(test)]
mod test {
    use speculoos::prelude::*;
    use proc_macro2::{Ident, Span};
    use syn::LitStr;
    use crate::html::{Html2, HtmlAttribute, HtmlElement};
    use crate::html_analyse::analyse;
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
        ]);

        let html: Html2 = analyse(input);

        assert_that(html.nodes.first().unwrap())
            .is_equal_to(HtmlElement::new(String::from("div"), Vec::new()))
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
        ]);

        let html: Html2 = analyse(input);

        assert_that(html.nodes.first().unwrap())
            .is_equal_to(HtmlElement::new(
                String::from("div"),
                vec![
                    HtmlAttribute::new(String::from("id"), Some(String::from("container")))
                ]
            ))
    }

    #[test]
    fn test_analyse_void_element() {

        let input = HtmlTokenStream::new(vec![
            HtmlToken::LessThan,
            HtmlToken::ElementStart { ident: Ident::new("div", Span::call_site()) },
            HtmlToken::Slash,
            HtmlToken::ElementEnd { ident: None },
            HtmlToken::GreaterThan,
        ]);

        let html: Html2 = analyse(input);

        assert_that(html.nodes.first().unwrap())
            .is_equal_to(HtmlElement::new(String::from("div"), Vec::new()))
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
        ]);

        let html: Html2 = analyse(input);

        assert_that(html.nodes.get(1).unwrap())
            .is_equal_to(HtmlElement::new(String::from("div"), Vec::new()));
        assert_that(html.nodes.get(0).unwrap())
            .is_equal_to(HtmlElement::new(String::from("p"), Vec::new()))
    }
}
