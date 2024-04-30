use crate::{css::CSSStyleSheet, node::Node};
use bevy::prelude::Entity;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

pub static GLOBAL_STATE: Lazy<Mutex<Node>> = Lazy::new(|| {
    let n = Node {
        children: Vec::new(),
        tag_name: "html".to_owned(),
        attributes: HashMap::new(),
        text: None,
        id: None,
        style_sheet_list: None,
    };
    Mutex::new(n)
});

#[derive(Debug)]
pub struct ChangeText {
    pub id: Entity,
    pub value: String,
}

#[derive(Debug)]
pub enum Action {
    ChangeTextAction(ChangeText),
    ChangeStyleAction(CSSStyleSheet),
}

pub struct ActionQueue {
    pub actions: Vec<Action>,
}

pub static GLOBAL_ACTION: Lazy<Mutex<ActionQueue>> = Lazy::new(|| {
    let n = ActionQueue {
        actions: Vec::new(),
    };
    Mutex::new(n)
});
