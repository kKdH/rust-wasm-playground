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

#[derive(PartialEq, Debug)]
pub struct HtmlElement {
    name: String,
    attributes: Vec<HtmlAttribute>
}

impl HtmlElement {

    pub fn new(name: String, attributes: Vec<HtmlAttribute>) -> HtmlElement {
        HtmlElement { name, attributes }
    }

    pub fn add_attribute(&mut self, attribute: HtmlAttribute) {
        self.attributes.push(attribute)
    }
}

#[derive(PartialEq, Debug)]
pub struct HtmlAttribute {
    pub name: String,
    pub value: Option<String>
}

impl HtmlAttribute {

    pub fn new(name: String, value: Option<String>) -> HtmlAttribute {
        HtmlAttribute { name, value }
    }
}

#[derive(PartialEq, Debug)]
pub struct HtmlText {
    value: String
}

impl HtmlText {

    pub fn new(value: String) -> HtmlText {
        HtmlText { value }
    }
}

pub struct Html2 {
    pub nodes: Vec<HtmlElement>
}

impl Html2 {

    pub fn new() -> Html2 {
        Html2 { nodes: Vec::new() }
    }

    pub fn root(&self) -> &HtmlElement {
        todo!()
    }

}
