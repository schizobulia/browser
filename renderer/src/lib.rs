mod action;
mod component;
mod css;
mod generate;

use bean::css::CSSRule;
use bean::node::{ElementText, Node};
use bean::qaq;
use bean::ui_state::UiState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy::prelude::*;
use component::input::{add_input_component, focus_node_style};
use component::tag_component::DomComponent;
use css::parse_css;
use generate::NodeResult;
use js_engine;
use scraper::{ElementRef, Html};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/**
 * Render the document
 */
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
            ..default()
        },
        background_color: Color::rgba(255.0, 255.0, 255.0, 1.0).into(),
        ..default()
    };

    let document = Html::parse_document(&html);
    let root_id = commands.spawn(root).id();
    let styles = traverse_html(
        document.root_element(),
        &mut commands,
        &asset_server,
        &mut js_runtime,
        root_id.clone(),
    );

    for style in styles {
        qaq::GLOBAL_ACTION
            .lock()
            .unwrap()
            .actions
            .push(qaq::Action::AddStyleSheetAction(parse_css(style)));
    }

    // let binding = qaq::GLOBAL_STATE.lock().unwrap().to_owned();
    // binding.pretty_print(0);
    // drop(binding);
}

/**
 * Create a node
 */
fn create_node(
    tag: String,
    attributes: HashMap<String, String>,
    commands: &mut Commands,
    parent_id: Entity,
    style: CSSRule,
    bundle: NodeBundle,
    mut dom: DomComponent,
    asset_server: &Res<AssetServer>,
) -> Node {
    let mut el_data = Node {
        children: Vec::new(),
        tag_name: tag.clone(),
        attributes: attributes,
        text: None,
        id: None,
        style_rules: None,
    };
    let id = commands
        .spawn(bundle)
        .insert(Interaction::Pressed)
        .insert(Interaction::Hovered)
        .id();
    if tag == "input" {
        let input_id = add_input_component(id, commands, &asset_server);
        dom.id = Some(input_id.clone());
    }

    commands.entity(parent_id).push_children(&vec![id.clone()]);
    commands.entity(id).insert(dom);
    el_data.id = Some(id.clone());
    el_data.style_rules = Some(style);
    el_data
}

/**
 * Traverse the html
 */
fn traverse_html(
    element: ElementRef,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    js_runtime: &mut js_engine::V8Runtime,
    root_id: Entity,
) -> Vec<String> {
    let mut styles = Vec::new();
    let root_html = Node {
        children: Vec::new(),
        tag_name: "root".to_owned(),
        attributes: HashMap::new(),
        text: None,
        id: Some(root_id),
        style_rules: None,
    };
    let mut root_node = qaq::GLOBAL_STATE.lock().unwrap();
    let html_tmp = get_arc_node(root_html);
    root_node.children.push(html_tmp.clone());
    drop(root_node);
    let mut inline_id = None;
    let mut stack = vec![(element, html_tmp)];
    while let Some((element, parent_node)) = stack.pop() {
        let tag = element.value().name().to_string();
        let res: NodeResult = generate::get_node_result(element, tag.clone());

        match res {
            NodeResult::Script(script) => {
                js_runtime.eval(Box::leak(script.clone().into_boxed_str()));
            }
            NodeResult::Style(style) => {
                styles.push(style);
            }
            NodeResult::Component(mut component) => {
                let parent_id = parent_node.lock().unwrap().id.unwrap().clone();
                let el_data: Node = create_node(
                    tag.clone(),
                    component.get_attributes(),
                    commands,
                    parent_id,
                    component.get_style_sheet(),
                    component.get_bundle(),
                    component.get_dom(),
                    asset_server,
                );
                let id = el_data.id.unwrap();
                let tmp = get_arc_node(el_data);
                parent_node.lock().unwrap().children.push(tmp.clone());
                if component.block_inline() == "inline" {
                    if inline_id.is_some() {
                        commands
                            .entity(inline_id.unwrap())
                            .push_children(&vec![id.clone()]);
                    } else {
                        inline_id = inline_node(commands);
                        commands
                            .entity(inline_id.unwrap())
                            .push_children(&vec![id.clone()]);
                        commands
                            .entity(parent_id)
                            .push_children(&vec![inline_id.unwrap()]);
                    }
                } else {
                    if inline_id.is_some() {
                        inline_id = None;
                    }
                }
                for child in element.children().rev() {
                    if let Some(child_element) = ElementRef::wrap(child) {
                        stack.push((child_element.clone(), tmp.clone()));
                    } else if child.value().is_text() {
                        let text = child
                            .value()
                            .as_text()
                            .unwrap()
                            .to_string()
                            .trim()
                            .to_string();
                        if text == "" {
                            continue;
                        }
                        let text_bundle = TextBundle::from_section(
                            text,
                            TextStyle {
                                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                ..component.get_style_text_inner()
                            },
                        );
                        let childern_id = commands.spawn(text_bundle).id();
                        commands
                            .entity(id)
                            .push_children(&vec![childern_id.clone()]);
                        tmp.lock().unwrap().text = Some(ElementText {
                            id: Some(childern_id),
                            text: child.value().as_text().unwrap().to_string(),
                        });
                    }
                }
            }
        };
    }

    styles
}

fn get_arc_node(node: Node) -> Arc<Mutex<Node>> {
    let node_nc = Arc::new(Mutex::new(node));
    Arc::clone(&node_nc)
}

fn inline_node(commands: &mut Commands) -> Option<Entity> {
    Some(
        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            })
            .id(),
    )
}

/**
 * Update the document by action
 * All the updates about dom on the page are here.
 */
pub fn update_document_by_action(
    // mut query_text: Query<&mut Text>,
    // mut query_style: Query<&mut Style>,
    mut query: Query<(&mut Text, &mut Style)>,
) {
    let mut binding_action = qaq::GLOBAL_ACTION.lock().unwrap();
    while binding_action.actions.len() > 0 {
        match binding_action.actions.remove(0) {
            qaq::Action::ChangeTextAction(change_text) => {
                action::change_text_action(&mut query, change_text);
            }
            qaq::Action::AddStyleSheetAction(style) => {
                action::add_style_sheet_action(style, &mut query);
            }
        }
    }
}

pub fn interaction_events(
    mut interaction_query: Query<(
        &Interaction,
        &mut Style,
        &mut BorderColor,
        Entity,
        &DomComponent,
    )>,
    mut text_query: Query<&mut Text>,
    mut ui_state: ResMut<UiState>,
) {
    for (interaction, mut style, mut border_color, _, dom) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                style.border = Default::default();
                if dom.tag_name == "input" {
                    if let Some(id) = dom.id {
                        ui_state.focus_node = Some(id.clone());
                        match text_query.get_mut(id) {
                            Ok(_) => {
                                let (border, color) = focus_node_style();
                                style.border = border;
                                border_color.0 = color;
                            }
                            Err(_) => {}
                        };
                    };
                }
            }
            Interaction::Hovered => {
                // println!("Hovered");
            }
            Interaction::None => {}
        }
    }
}

pub fn listen_keyboard_input_events(
    mut events: EventReader<KeyboardInput>,
    mut edit_text: Query<&mut Text>,
    ui_state: ResMut<UiState>,
) {
    for event in events.read() {
        if event.state == ButtonState::Released {
            continue;
        }
        match ui_state.focus_node {
            Some(id) => match edit_text.get_mut(id) {
                Ok(mut text) => match &event.logical_key {
                    Key::Enter => {}
                    Key::Space => {
                        text.sections[0].value.push(' ');
                    }
                    Key::Backspace => {
                        text.sections[0].value.pop();
                    }
                    Key::Character(character) => {
                        text.sections[0].value.push_str(character);
                    }
                    _ => continue,
                },
                Err(_) => {}
            },
            None => {}
        }
    }
}
