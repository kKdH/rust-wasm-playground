use proc_macro2::{Ident, Span};
use vdom::VTree;

pub struct Html {
    root: HtmlNode,
    pub tree: VTree
}

impl Html {
    pub fn new(tree: VTree) -> Html {
        Html {
            root: HtmlNode::Element { name: Ident::new("undefined", Span::call_site()) },
            tree
        }
    }
}

#[derive(PartialEq, Debug)]
enum HtmlNode {
    Element { name: Ident },
    Text { value: String }
}

struct HtmlElement {}

struct HtmlText {}
