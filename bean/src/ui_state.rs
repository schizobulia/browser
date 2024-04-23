use bevy::prelude::Resource;
use crate::node::Node;

#[derive(Resource)]
pub struct UiState {
    pub name: String,
    pub document: Vec<Node>
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            document: Vec::new()
        }
    }
}
