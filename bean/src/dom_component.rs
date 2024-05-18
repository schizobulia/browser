use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;

#[derive(Component, Clone, Debug)]
pub struct DomComponent {
    pub id: Option<Entity>,
    pub tag_name: String,
}
