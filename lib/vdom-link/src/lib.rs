use web_sys::{Document, Element};

use vdom::{VItem, VNode};

pub trait VNodeLink {

    fn upsert(&self, document: &Document) -> Element;
}

impl VNodeLink for VNode {

    fn upsert(&self, document: &Document) -> Element {
        let id: String = self.id.into();
        let item: &Option<VItem> = &self.item;
        let element = match document.get_element_by_id(id.as_str()) {
            None => {
                match item {
                    None => { panic!("No item") }
                    Some(VItem::Element { name, attributes}) => {
                        let element = document.create_element(name).expect("element created");
                        element.set_id(id.as_str());
                        attributes.iter().for_each(|(name, value)| {
                            element.set_attribute(name.as_str(), value.as_str());
                        });
                        element
                    }
                    Some(VItem::Text { .. }) => {
                        panic!("VItem::Text not supported!")
                    }
                }
            },
            Some(element) => element,
        };
        element.set_text_content(Some("Hello World"));
        element
    }
}
