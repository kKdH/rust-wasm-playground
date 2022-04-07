use web_sys::{Document, Element};

use vdom::VNode;

pub trait VNodeLink {

    fn upsert(&self, document: &Document) -> Element;
}

impl VNodeLink for VNode {

    fn upsert(&self, document: &Document) -> Element {
        let id: String = self.id.into();
        let name: &str = &self.kind;
        let element = match document.get_element_by_id(id.as_str()) {
            None => {
                let element =
                    document.create_element(name)
                        .expect("element created");
                element.set_id(id.as_str());
                element
            },
            Some(element) => element,
        };
        element.set_text_content(Some("Hello World"));
        element
    }
}
