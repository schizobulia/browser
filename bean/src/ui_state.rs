use bevy::prelude::Resource;

/**
 * bevy ui state
 */
#[derive(Resource)]
pub struct UiState {
    pub name: String,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            name: "http://49.232.147.237/test/index.html".to_owned(),
        }
    }
}
