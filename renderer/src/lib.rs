mod action;
mod component;
mod css;
mod generate;

use std::collections::HashMap;

use bevy::prelude::*;
// use bevy_egui::EguiContexts;
use bean::node::{ElementText, Node};
use bean::qaq;
use bean::ui_state::UiState;
use css::parse_css;
use generate::NodeResult;
use js_engine;
use scraper::{ElementRef, Html};
#[derive(Component)]
struct AnimateTranslation;

pub fn render_document(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    _ui_state: ResMut<UiState>,
    html: String,
) {
    let mut js_runtime = js_engine::V8Runtime::new();
    js_runtime.init_global();
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
    let mut styles = Vec::new();
    
    let document = Html::parse_document(&html);
    let root_id = commands.spawn(root).id();

    traverse_html(
        document.root_element(),
        &mut commands,
        &asset_server,
        &mut js_runtime,
        &mut styles,
        root_id.clone(),
    );

    for style in styles {
        qaq::GLOBAL_ACTION
            .lock()
            .unwrap()
            .actions
            .push(qaq::Action::ChangeStyleAction(parse_css(style)));
    }
}

fn traverse_html(
    element: ElementRef,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    js_runtime: &mut js_engine::V8Runtime,
    styles: &mut Vec<String>,
    root_id: Entity,
) {
    let mut rid = root_id.clone();
    let mut stack = vec![element];
    while let Some(element) = stack.pop() {
        let res: NodeResult = generate::get_node_result(element);
        match res {
            NodeResult::Script(script) => {
                js_runtime.eval(Box::leak(script.clone().into_boxed_str()));
            }
            NodeResult::Style(style) => {
                styles.push(style);
            }
            NodeResult::Div(bundle, style, text_style) => {
                let mut binding = qaq::GLOBAL_STATE.lock().unwrap();
                let list: &mut Vec<Node> = binding.children.as_mut();
                let tag = element.value().name().to_string();
                let mut attributes = HashMap::new();
                element.value().attrs.clone().iter().for_each(|attr| {
                    attributes.insert(attr.0.local.to_string(), attr.1.to_string());
                });
                let mut el_data: Node = Node {
                    children: Vec::new(),
                    tag_name: tag,
                    attributes: attributes,
                    text: None,
                    id: None,
                    style_sheet_list: None,
                };
                // binding = qaq::GLOBAL_STATE.lock().unwrap(); 
                let id = commands.spawn(bundle).id();
                commands.entity(rid).push_children(&vec![id.clone()]);
                el_data.id = Some(id.clone());
                el_data.style_sheet_list = Some(style);
                rid = id.clone();
                for child in element.children().rev() {
                    if let Some(child_element) = ElementRef::wrap(child) {
                        stack.push(child_element.clone());
                    } else if child.value().is_text() {
                        let text = child.value().as_text().unwrap().to_string();
                        if text.trim().is_empty() {
                            continue;
                        }
                        let text_bundle = TextBundle::from_section(
                            text,
                            TextStyle {
                                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                ..text_style
                            },
                        );
                        let childern_id = commands.spawn(text_bundle).id();
                        commands.entity(id).push_children(&vec![childern_id.clone()]);
                        el_data.text = Some(ElementText {
                            id: Some(childern_id),
                            text: child.value().as_text().unwrap().to_string(),
                        });
                    }
                }
                list.push(el_data);
                drop(binding);
            }
        };

    }
}

pub fn update_node_text(
    // mut query_text: Query<&mut Text>,
    // mut query_style: Query<&mut Style>,
    mut query: Query<(&mut Text, &mut Style)>,
) {
    let mut list = qaq::GLOBAL_STATE.lock().unwrap().children.clone();
    let mut binding_action = qaq::GLOBAL_ACTION.lock().unwrap();
    while binding_action.actions.len() > 0 {
        match binding_action.actions.remove(0) {
            qaq::Action::ChangeTextAction(change_text) => {
                action::change_text_action(&mut query, change_text);
            }
            qaq::Action::ChangeStyleAction(style) => {
                action::change_style_action(style, &mut list, &mut query);
            }
        }
    }
}
