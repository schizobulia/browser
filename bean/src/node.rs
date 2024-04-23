use bevy::{prelude::Entity};

#[derive(Debug, Clone)]
pub struct Node {
    pub children: Vec<Node>,
    pub tag_name: String,
    pub attributes: Vec<(String, String)>,
    pub text: Option<ElementText>,
    pub id: Option<Entity>,
}

#[derive(Debug, Clone)]
pub struct ElementText {
    pub id: Option<Entity>,
    pub text: String,
}

pub fn get_children_by_tag_name<'a>(tag_name: &str, list: &'a mut Vec<Node>) -> Vec<Entity> {
    let mut result = Vec::new();
    let mut queue = list.iter_mut().collect::<Vec<_>>();

    while !queue.is_empty() {
        let node = queue.remove(0);
        if node.tag_name == tag_name {
            result.push(node.clone().id.unwrap());
        }
        queue.extend(node.children.iter_mut().collect::<Vec<_>>());
    }
    result
}

pub fn get_node_by_id(list: &mut Vec<Node>, id: Entity) -> Option<&mut Node> {
    let mut queue = list.iter_mut().collect::<Vec<_>>();
    while !queue.is_empty() {
        let node = queue.remove(0);
        if node.id.unwrap() == id {
            return Some(node);
        }
        queue.extend(node.children.iter_mut().collect::<Vec<_>>());
    }
    None
}