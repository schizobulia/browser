use crate::css::CSSRule;
use bevy::prelude::Entity;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

/**
 * The structure of dom in memory
 */
#[derive(Debug, Clone)]
pub struct Node {
    pub children: Vec<Arc<Mutex<Node>>>,
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
    pub text: Option<ElementText>,
    pub id: Option<Entity>,
    pub style_rules: Option<CSSRule>,
}

/**
 * Text node
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementText {
    pub id: Option<Entity>,
    pub text: String,
}

impl Node {
    pub fn pretty_print(&self, depth: usize) {
        let indent = "  ".repeat(depth);
        println!("{}<{}>", indent, self.tag_name);
        for child in &self.children {
            child.lock().unwrap().pretty_print(depth + 1);
        }
        if let Some(text) = &self.text {
            println!("{}  {}", indent, text.text);
        }
        println!("{}</{}>", indent, self.tag_name);
    }

    pub fn get_node_by_id(&self, id: u64) -> Option<Arc<Mutex<Node>>> {
        let mut queue = self.children.to_vec();
        while let Some(node_arc) = queue.pop() {
            let node = node_arc.lock().unwrap();
            if node.id == Some(set_u64_to_entity(id)) {
                return Some(node_arc.clone());
            }
            // 将子节点添加到待访问列表
            queue.extend(node.children.iter().cloned());
        }
        None
    }

    pub fn get_node_by_tag_id(&self, id: String) -> Option<Arc<Mutex<Node>>> {
        let mut queue = self.children.to_vec();
        while let Some(node_arc) = queue.pop() {
            let node = node_arc.lock().unwrap();
            if node
                .attributes
                .iter()
                .any(|(key, value)| key == "id" && value == &id)
            {
                return Some(node_arc.clone());
            }
            // 将子节点添加到待访问列表
            queue.extend(node.children.iter().cloned());
        }
        None
    }

    pub fn get_children_by_tag_name(&self, tag_name: &str) -> Vec<Entity> {
        let mut matching_ids = Vec::new();
        let mut nodes_to_visit = self.children.clone();

        while let Some(node_arc) = nodes_to_visit.pop() {
            let node = node_arc.lock().unwrap();
            if node.tag_name == tag_name {
                if let Some(id) = node.id {
                    matching_ids.push(id);
                }
            }
            for child in &node.children {
                nodes_to_visit.push(child.clone());
            }
        }
        matching_ids
    }

    pub fn to_json(&self) -> String {
        let value = json!({
            "tag_name": self.tag_name,
            "attributes": self.attributes,
            "text": self.text,
            "id": self.id,
            "style_rules": self.style_rules,
        });
        value.to_string()
    }
}

// pub fn get_node_by_id(list: &mut Vec<Node>, id: Entity) -> Option<&mut Node> {
//     let mut queue = list.iter_mut().collect::<Vec<_>>();
//     while !queue.is_empty() {
//         let node = queue.remove(0);
//         if node.id.unwrap() == id {
//             return Some(node);
//         }
//         queue.extend(node.children.iter_mut().collect::<Vec<_>>());
//     }
//     None
// }

pub fn set_u64_to_entity(id: u64) -> Entity {
    Entity::from_bits(id)
}
