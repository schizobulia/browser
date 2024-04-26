use once_cell::sync::Lazy;
use crate::node::Node;
use std::{sync::Mutex};
use bevy::{prelude::Entity};

pub static GLOBAL_STATE: Lazy<Mutex<Node>> = Lazy::new(|| {
    let n = Node {
        children: Vec::new(),
        tag_name: "html".to_owned(),
        attributes: Vec::new(),
        text: None,
        id: None,
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
