use bevy::{prelude::Resource};

use crate::dom_component::DomComponent;

/**
 * bevy ui state
 */
#[derive(Resource)]
pub struct UiState {
    pub name: String,
    pub focus_node: Option<DomComponent>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            name: "http://49.232.147.237/test/index.html".to_owned(),
            focus_node: None
        }
    }
}
