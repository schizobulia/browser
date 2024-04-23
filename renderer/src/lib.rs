mod component;

use bevy::{prelude::*};
// use bevy_egui::EguiContexts;
use scraper::{ElementRef, Html};
use bean::node::{Node, ElementText, get_children_by_tag_name, get_node_by_id};
use bean::ui_state::UiState;

#[derive(Component)]
struct AnimateTranslation;

pub fn render_document(mut commands: Commands, asset_server: Res<AssetServer>,
    mut ui_state: ResMut<UiState>, html: String) {
    let root = NodeBundle {
        style: Style {
            top: Val::Px(25.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        background_color: Color::rgba(255.0, 255.0, 255.0, 1.0).into(),
        ..default()
    };
    commands.spawn(root).with_children(|parent: &mut ChildBuilder<'_>| {
        let document = Html::parse_document(&html);
        let node = traverse_html(document.root_element(), parent, &asset_server);
        ui_state.document = vec![node];
        get_children_by_tag_name("p", &mut ui_state.document).iter_mut().for_each(|a| {
            let n = get_node_by_id(&mut ui_state.document, a.clone());
            match n {
                Some(node) => {
                    // match &mut node.text {
                    //     Some(text) => {
                    //         text.text = "Hello, world!".to_string();
                    //     },
                    //     None => {}
                    // }
                    println!("{:?}", node.text);
                    println!("{:?}", node.children);
                },
                None => {}
            }
        });
        // println!("{:?}", ui_state.document);
    });
}

fn traverse_html(element: ElementRef, commands: &mut ChildBuilder<'_>, asset_server: &Res<AssetServer>) -> Node {
    let tag = element.value().name().to_string();
    let mut children: Vec<Node> = Vec::new();
    let mut el_data: Node = Node {
        children: Vec::new(),
        tag_name: tag,
        attributes: Vec::new(),
        text: None,
        id: None,
    };
    let root = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    };
    let id = commands.spawn(root).with_children(|parent: &mut ChildBuilder<'_>| {
        for child in element.children() {
            if let Some(child_element) = ElementRef::wrap(child) {
                children.push(traverse_html(child_element, parent, asset_server));
            } else if child.value().is_text() {
                let text = child.value().as_text().unwrap().to_string();
                let text_bundle = TextBundle::from_section(
                    &text,
                    TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        color: Color::BLACK,
                        ..default()
                    },
                );
                let childern_id = parent.spawn(text_bundle).id();
                el_data.text = Some(ElementText { id: Some(childern_id), text });
            }
        }
    }).id();
    el_data.children = children;
    el_data.id = Some(id);
    el_data
}

pub fn update_document(
    // mut contexts: EguiContexts, mut ui_state: ResMut<UiState>, mut query: Query<&mut Text>
){
    // for mut text in &mut query {
    //     text.sections[0].value = ui_state.name.clone();
    // }
    // query.get_mut(entity)
}
