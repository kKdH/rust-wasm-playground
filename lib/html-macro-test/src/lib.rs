mod ui;

#[cfg(test)]
mod test {
    use speculoos::prelude::*;
    use html_macro::html;
    use vdom::VTree;

    #[test]
    fn test_parse() {

        let parsed_tree: VTree = html! {
            <div>
                <p></p>
            </div>
        };

        let parsed_root = parsed_tree.get_root();

        assert_that!(&parsed_tree.nodes()).has_length(2);
        assert_that!(&parsed_tree.nodes())
            .matching_contains(|node| {
                "div" == (*node).kind.as_str()
            });
        assert_that!(&parsed_tree.nodes())
            .matching_contains(|node| {
                "p" == (*node).kind.as_str()
            });
        assert_that!(parsed_root)
            .is_some();
        assert_that!(parsed_tree.get_node(&parsed_root.unwrap()).unwrap().kind)
            .is_equal_to(String::from("div"));
        assert_that!(&parsed_tree.children(&parsed_root.unwrap()))
            .matching_contains(|node| {
                "p" == (*node).kind.as_str()
            });
    }
}
