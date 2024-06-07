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
            match child.lock() {
                Ok(data) => {
                    data.pretty_print(depth + 1);
                }
                Err(_) => {
                    println!("{}  <Error: Deadlock>", indent);
                    continue;
                }
            }
        }
        if let Some(text) = &self.text {
            println!("{}  {}", indent, text.text);
        }
        println!("{}</{}>", indent, self.tag_name);
    }

    pub fn get_node_by_id(&self, id: u64) -> Option<Arc<Mutex<Node>>> {
        let mut queue = self.children.to_vec();
        while let Some(node_arc) = queue.pop() {
            match node_arc.lock() {
                Ok(node) => {
                    if node.id == Some(set_u64_to_entity(id)) {
                        return Some(node_arc.clone());
                    }
                    // 将子节点添加到待访问列表
                    queue.extend(node.children.iter().cloned());
                }
                Err(_) => {
                    continue;
                }
            }
        }
        None
    }

    pub fn get_node_by_tag_id(&self, id: String) -> Option<Arc<Mutex<Node>>> {
        let mut queue = self.children.to_vec();
        while let Some(node_arc) = queue.pop() {
            match node_arc.lock() {
                Ok(node) => {
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
                Err(_) => {
                    continue;
                }
            }
        }
        None
    }

    pub fn get_children_by_tag_name(&self, tag_name: &str) -> Vec<Entity> {
        let mut matching_ids = Vec::new();
        let mut nodes_to_visit = self.children.clone();

        while let Some(node_arc) = nodes_to_visit.pop() {
            match node_arc.lock() {
                Ok(node) => {
                    if node.tag_name == tag_name {
                        if let Some(id) = node.id {
                            matching_ids.push(id);
                        }
                    }
                    for child in &node.children {
                        nodes_to_visit.push(child.clone());
                    }
                }
                Err(_) => {
                    continue;
                }
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

pub fn set_u64_to_entity(id: u64) -> Entity {
    Entity::from_bits(id)
}
