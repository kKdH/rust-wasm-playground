
#[derive(PartialEq, Debug)]
pub struct Html {
    root: HtmlElement
}

impl Html {

    pub fn new(root: HtmlElement) -> Html {
        Html { root }
    }

    pub fn root(&self) -> &HtmlElement {
        &self.root
    }
}

#[derive(PartialEq, Debug)]
pub struct HtmlElement {
    name: String,
    attributes: Vec<HtmlAttribute>,
    children: Vec<HtmlElement>,
    text: Option<String>,
}

impl HtmlElement {

    pub fn new(name: String, attributes: Vec<HtmlAttribute>, children: Vec<HtmlElement>, text: Option<String>) -> HtmlElement {
        HtmlElement { name, attributes, children, text }
    }

    pub fn add_attribute(&mut self, attribute: HtmlAttribute) {
        self.attributes.push(attribute)
    }

    pub fn add_child(&mut self, child: HtmlElement) {
        self.children.push(child)
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_text(&mut self, text: Option<String>) {
        self.text = text
    }

    pub fn get_text(&self) -> &Option<String> {
        &self.text
    }

    pub fn children(&self) -> &Vec<HtmlElement> {
        &self.children
    }

    pub fn attributes(&self) -> &Vec<HtmlAttribute> {
        &self.attributes
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct HtmlAttribute {
    pub name: String,
    pub value: Option<String>
}

impl HtmlAttribute {

    pub fn new(name: String, value: Option<String>) -> HtmlAttribute {
        HtmlAttribute { name, value }
    }
}
