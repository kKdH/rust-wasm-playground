extern crate wasm_bindgen;

use log::info;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Element, Event, EventTarget, HtmlButtonElement, HtmlElement};
use web_sys::console::info;

use html_macro::html;
use vdom::{VNode, VTree};
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
            <p></p>
            <button></button>
        </div>
    };

    tree.nodes().iter().for_each(|node| {
        let element = (*node).upsert(&document);
        app.append_child(&element);
    });

    // let divElement = tree.upsert(&document);
    //
    // app.append_child(&divElement)
    //     .expect("append child");

    app.append_child(&button)
        .expect("append child");

    info!("app: {:?}", app);
}
