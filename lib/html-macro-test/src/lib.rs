mod ui;

#[cfg(test)]
mod test {
    use speculoos::prelude::*;
    use html_macro::html;
    use vdom::{VItem, VTree};

    #[test]
    fn test_parse() {

        let parsed_tree: VTree = html! {
            <div class="container">
                <p></p>
            </div>
        };

        let parsed_root = parsed_tree.get_root();

        assert_that!(&parsed_tree.nodes()).has_length(2);
        assert_that!(&parsed_tree.nodes())
            .matching_contains(|node| {
                match &node.item {
                    Some(VItem::Element { name, .. }) => "div" == name,
                    _ => false
                }
            });
        assert_that!(&parsed_tree.nodes())
            .matching_contains(|node| {
                match &node.item {
                    Some(VItem::Element { name, .. }) => "p" == name,
                    _ => false
                }
            });
        assert_that!(parsed_root)
            .is_some();
        assert_that!(parsed_tree.get_node(&parsed_root.unwrap()).unwrap().item)
            .is_equal_to(Some(VItem::Element {
                name: String::from("div"),
                attributes: vec![("class".into(), "container".into())],
                text: None
            }));
        assert_that!(&parsed_tree.children(&parsed_root.unwrap()))
            .matching_contains(|node| {
                match &node.item {
                    Some(VItem::Element { name, .. }) => "p" == name,
                    _ => false
                }
            });
    }
}
