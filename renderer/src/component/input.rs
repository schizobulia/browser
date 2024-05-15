use bevy::prelude::*;

pub fn init_style(bundle: &mut NodeBundle, style: &mut Style, style_text_inner: &mut TextStyle) {
    bundle.background_color = BackgroundColor(Color::WHITE);
    bundle.border_color = BorderColor(Color::BLACK);

    style.width = Val::Px(147.0);
    style.height = Val::Px(13.0);
    style.align_items = AlignItems::Center;
    style.border = UiRect {
        left: Val::Px(0.3),
        right: Val::Px(0.3),
        top: Val::Px(0.3),
        bottom: Val::Px(0.3),
    };
    style_text_inner.font_size = 12.0;
}

pub fn add_input_component(
    parent_id: Entity,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let text_bundle = TextBundle::from_section(
        "123",
        TextStyle {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 12.0,
            color: Color::BLACK,
            ..Default::default()
        },
    );
    let childern_id = commands.spawn(text_bundle).id();
    commands
        .entity(parent_id)
        .push_children(&vec![childern_id.clone()]);
    childern_id
}

pub fn focus_node_style() -> (UiRect, Color) {
    let border = UiRect {
        left: Val::Px(1.3),
        right: Val::Px(1.3),
        top: Val::Px(1.3),
        bottom: Val::Px(1.3),
    };
    (border, Color::BLUE)
}
