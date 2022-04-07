extern crate wasm_bindgen;

use std::collections::{HashMap, VecDeque};
use log::info;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Element, Event, HtmlButtonElement};

use html_macro::html;
use vdom::{VNode, VRef, VTree};
use vdom_link::VNodeLink;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let app = document.get_elements_by_tag_name("app").item(0).expect("docuemnt should have app");

    let mut clicks = 0;

    let div = document.create_element("div")
        .expect("div created");

    div.set_inner_html(&String::from(format!("Click count: {:?}", clicks)));
    div.set_class_name("title has-text-grey-lighter");
    app.append_child(&div);

    let button: Element = {
        let element = document.create_element("button").expect("button created");
        let button_element: &HtmlButtonElement = element.dyn_ref::<HtmlButtonElement>().expect("has to be a button");
        element.set_inner_html("Click Me!");
        element.set_class_name("button");

        let callback = Closure::wrap(Box::new(
            move |event: Event| {
                clicks += 1;
                div.set_inner_html(&String::from(format!("Click count: {:?}", clicks)));
                info!("Clicked!: {:?}", clicks);
            }
        ) as Box<dyn FnMut(Event)>);

        button_element.set_onclick(Some(callback.as_ref().unchecked_ref()));
        callback.forget(); // TODO: Remove call to forget (memory leak).
        element
    };

    let tree: VTree = html! {
        <div>
            <div>
                <div></div>
                <div>
                    <label></label>
                    <div>
                        <input></input>
                    </div>
                </div>
                <div>
                    <div>
                        <button></button>
                    </div>
                </div>
            </div>
        </div>
    };

    let mut elements: HashMap<VRef, Element> = HashMap::new();
    let mut render_queue: VecDeque<VRef> = VecDeque::new();
    let root_node = tree.get_root().expect("Root node.");

    render_queue.push_back(root_node);

    while !render_queue.is_empty() {
        let node_ref = render_queue.pop_front().expect("Ref");
        let node: VNode = tree.get_node(&node_ref).expect("Node");
        let element = node.upsert(&document);

        if let Some(parent_ref) = tree.parent(&node_ref) {
            let parent_element = elements.get(parent_ref).expect("Parent element");
            parent_element.append_child(&element);
        }

        info!("Node: {:?}", node);
        tree.children(&node_ref).iter().for_each(|child_ref| {
            render_queue.push_back(child_ref.id)
        });
        elements.insert(node_ref, element);
    }

    let root_element = elements.get(&root_node).expect("Element");
    app.append_child(&root_element);

    info!("app: {:?}", app);
}
