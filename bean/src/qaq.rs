use crate::{css::CSSStyleSheet, node::Node};
use bevy::prelude::Entity;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/**
 * Global state management
 */
pub static GLOBAL_STATE: Lazy<Arc<Mutex<Node>>> = Lazy::new(|| {
    let n = Node {
        children: Vec::new(),
        tag_name: "0".to_owned(),
        attributes: HashMap::new(),
        text: None,
        id: None,
        style_rules: None,
    };
    Arc::new(Mutex::new(n))
});

#[derive(Debug)]
pub struct ChangeText {
    pub id: Entity,
    pub value: String,
}

/**
 * Action type
 * Different action means different tasks.
 */
#[derive(Debug)]
pub enum Action {
    ChangeTextAction(ChangeText),
    AddStyleSheetAction(CSSStyleSheet),
}

pub struct ActionQueue {
    pub actions: Vec<Action>,
}

/**
 * Global action queue
 */
pub static GLOBAL_ACTION: Lazy<Mutex<ActionQueue>> = Lazy::new(|| {
    let n = ActionQueue {
        actions: Vec::new(),
    };
    Mutex::new(n)
});
