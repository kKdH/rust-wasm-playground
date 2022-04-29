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
}

#[derive(PartialEq, Debug)]
pub struct HtmlElement {
    name: String,
    attributes: Vec<HtmlAttribute>,
    children: Vec<HtmlElement>
}

impl HtmlElement {

    pub fn new(name: String, attributes: Vec<HtmlAttribute>, children: Vec<HtmlElement>) -> HtmlElement {
        HtmlElement { name, attributes, children }
    }

    pub fn add_attribute(&mut self, attribute: HtmlAttribute) {
        self.attributes.push(attribute)
    }

    pub fn add_child(&mut self, child: HtmlElement) {
        self.children.push(child)
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

#[derive(PartialEq, Debug)]
pub struct Html2 {
    root: HtmlElement
}

impl Html2 {

    pub fn new(root: HtmlElement) -> Html2 {
        Html2 { root }
    }

    pub fn root(&self) -> &HtmlElement {
        &self.root
    }
}
