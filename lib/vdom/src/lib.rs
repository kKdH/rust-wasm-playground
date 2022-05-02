use std::collections::HashMap;

use uuid::Uuid;

pub struct Error;

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub struct VRef {
    id: Uuid
}

impl VRef {

    pub fn random() -> VRef {
        VRef { id: Uuid::new_v4() }
    }

    pub fn from_string(uuid: String) -> Result<VRef, Error> {
        match Uuid::parse_str(uuid.as_str()) {
            Ok(uuid) => {
                Ok(VRef { id: uuid })
            }
            Err(_) => {
                Err(Error {})
            }
        }
    }
}

impl From<VRef> for String {
    fn from(vref: VRef) -> Self {
        vref.id.to_string()
    }
}

impl From<&VRef> for String {
    fn from(vref: &VRef) -> Self {
        vref.id.to_string()
    }
}

impl From<&VNode> for VRef {
    fn from(node: &VNode) -> Self {
        node.id
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum VItem {
    Element {
        name: String
    },
    Text {
        value: String
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct VNode {
    pub id: VRef,
    pub parent: Option<VRef>,
    pub children: Vec<VRef>,
    pub item: Option<VItem>,
}

impl VNode {

    pub fn new(id: VRef) -> VNode {
        VNode {
            id,
            parent: None,
            children: Vec::new(),
            item: None,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct VTree {
    nodes: HashMap<VRef, VNode>,
    parents: HashMap<VRef, VRef>,
    children: HashMap<VRef, Vec<VRef>>,
    root: Option<VRef>
}

impl VTree {

    pub fn new() -> VTree {
        VTree {
            nodes: HashMap::new(),
            parents: HashMap::new(),
            children: HashMap::new(),
            root: None
        }
    }

    pub fn create_node(&mut self, node_ref: &VRef) -> VRef {
        let node = VNode::new(*node_ref);
        self.nodes.insert(*node_ref, node);

        *node_ref
    }

    pub fn create_random_node(&mut self) -> VRef {
        self.create_node(&VRef::random())
    }

    pub fn set_root(&mut self, node: &VRef) {
        self.root = Some(*node)
    }

    pub fn get_root(&self) -> Option<VRef> {
        self.root
    }

    pub fn has_root(&self) -> bool {
        self.root.is_some()
    }

    pub fn append_child(&mut self, parent: &VRef, child: &VRef) {

        if let Some(previous_parent) = self.parents.remove(child) {
            VTree::_remove_child(&mut self.children, &previous_parent, child);
        }

        self.parents.insert(*child, *parent);
        self.children.entry(*parent)
            .or_insert_with(Vec::new)
            .push(*child);
    }

    pub fn remove_child(&mut self, parent: &VRef, child: &VRef) {
        let mapped_parent = self.parents.remove(child);
        let mapped_childrens = self.children.get_mut(&parent)
            .map(|children| {
                let index = children.iter().position(|vref| *vref == *child);
                (Some(children), index)
            });

        match (mapped_parent, mapped_childrens) {
            (Some(_), Some((Some(childrens), Some(child_index)))) => {
                childrens.remove(child_index);
                if childrens.is_empty() {
                    self.children.remove(parent);
                }
            }
            _ => {
                // TODO: handle all cases separately.
                panic!("Invalid parent/child mapping!")
            }
        }
    }

    pub fn nodes(&self) -> Vec<&VNode> {
        Vec::from_iter(self.nodes.values().into_iter())
    }

    pub fn get_node(&self, node: &VRef) -> Option<VNode> {
        self.nodes.get(node).cloned()
    }

    pub fn update_node(&mut self, node: &VRef, update_fn: Box<dyn FnOnce(&mut VNode) -> ()>) {
        if let Some(node) = self.nodes.get_mut(node) {
            update_fn(node)
        }
    }

    pub fn parent(&self, child: &VRef) -> Option<&VRef> {
        self.parents.get(child)
    }

    pub fn children(&self, parent: &VRef) -> Vec<&VNode> {
        match self.children.get(parent) {
            None => {
                Vec::new()
            }
            Some(children) => {
                children.iter().map(|vref| {
                    self.nodes
                        .get(vref)
                        .expect("Could not resolve vref")
                }).collect()
            }
        }
    }

    fn _remove_child(children: &mut HashMap<VRef, Vec<VRef>>, parent: &VRef, child: &VRef) {
        if let Some(children) = children.get_mut(&parent) {
            if let Some(index) = children.iter().position(|vref| *vref == *child ) {
                children.remove(index);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use speculoos::prelude::*;

    use crate::{VItem, VRef, VTree};

    #[test]
    fn test_parent_child_relationship() {

        let mut tree = VTree::new();
        let node_a = tree.create_random_node();
        let node_b = tree.create_random_node();
        let node_c = tree.create_random_node();
        let node_d = tree.create_random_node();

        tree.append_child(&node_a, &node_b);

        assert_that!(&tree.nodes).has_length(4);
        assert_that!(&tree.parents)
            .contains_entry(node_b, node_a);
        assert_that!(&tree.children)
            .contains_entry(node_a, vec![node_b]);

        tree.remove_child(&node_a, &node_b);

        assert_that!(&tree.nodes).has_length(4);
        assert_that!(&tree.parents).is_empty();
        assert_that!(&tree.children).is_empty();

        tree.append_child(&node_a, &node_b);
        tree.append_child(&node_a, &node_c);
        tree.append_child(&node_b, &node_d);

        assert_that!(&tree.children)
            .contains_entry(node_a, vec![node_b, node_c]);
        assert_that!(&tree.children)
            .contains_entry(node_b, vec![node_d]);
    }

    #[test]
    fn test_nodes_iter() {
        let mut tree = VTree::new();
        let node_a = tree.create_random_node();
        let node_b = tree.create_random_node();
        let node_c = tree.create_random_node();
        let node_d = tree.create_random_node();

        let expected: Vec<&VRef> = vec![&node_a, &node_b, &node_c, &node_d];

        assert_that!(tree.nodes()).has_length(4);
        assert_that!(tree.nodes()
            .iter()
            .map(|node| (*node).id)
            .collect::<Vec<VRef>>())
            .contains_all_of(&expected);
    }

    #[test]
    fn test_update_node() {

        let mut tree = VTree::new();
        let node_a = tree.create_random_node();
        let node_b = tree.create_random_node();
        let node_c = tree.create_random_node();
        let node_d = tree.create_random_node();

        tree.append_child(&node_a, &node_b);
        tree.append_child(&node_a, &node_c);
        tree.append_child(&node_b, &node_d);


        tree.update_node(&node_c, Box::new(|node| {
            node.item = Some(VItem::Text { value: String::from("div") })
        }));

        let item = tree.get_node(&node_c).unwrap().item.unwrap();

        assert_that!(&item)
            .is_equal_to(VItem::Text { value: String::from("div") })

    }
}
