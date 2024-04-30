use crate::node::Node;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct UiState {
    pub name: String,
    pub document: Vec<Node>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            name: "http://49.232.147.237/test/index.html".to_owned(),
            document: Vec::new(),
        }
    }
}
