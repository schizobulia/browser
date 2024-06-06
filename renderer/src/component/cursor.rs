use bevy::{
    ecs::{component::Component, entity::Entity, system::Commands},
    render::{color::Color, view::Visibility},
    ui::{node_bundles::NodeBundle, BackgroundColor, Style, Val},
};
use chrono::Utc;

#[derive(Component)]
pub struct Cursor {
    pub show: bool,
    pub time: i64,
}

pub fn insert_cursor(commands: &mut Commands) -> Entity {
    let bundle = NodeBundle {
        background_color: BackgroundColor(Color::BLACK),
        style: Style {
            width: Val::Px(1.0),
            height: Val::Percent(100.0),
            ..Default::default()
        },
        visibility: Visibility::Hidden,
        ..Default::default()
    };
    let id = commands.spawn(bundle).id();
    let cursor = Cursor {
        show: false,
        time: Utc::now().timestamp(),
    };
    commands.entity(id).insert(cursor);
    id
}
