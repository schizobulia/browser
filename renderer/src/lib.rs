mod action;
mod component;
mod css;
mod generate;

use bean::css::CSSRule;
use bevy::prelude::*;
use bean::node::{ElementText, Node};
use bean::qaq;
use bean::ui_state::UiState;
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
            justify_content: JustifyContent::SpaceBetween,
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

    let binding = qaq::GLOBAL_STATE.lock().unwrap().to_owned();
    binding.pretty_print(0);
    drop(binding);
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
    bundle: NodeBundle) -> Node {
    let mut el_data = Node {
        children: Vec::new(),
        tag_name: tag.clone(),
        attributes: attributes,
        text: None,
        id: None,
        style_rules: None,
    };
    let id = commands.spawn(bundle).id();
    commands.entity(parent_id).push_children(&vec![id.clone()]);
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
            NodeResult::Component(bundle, style, text_style, attributes) => {
                let el_data: Node = create_node(tag.clone(), attributes, commands, parent_node.lock().unwrap().id.unwrap().clone(), style, bundle);
                let id = el_data.id.unwrap();
                let tmp = get_arc_node(el_data);
                parent_node.lock().unwrap().children.push(tmp.clone());
                for child in element.children().rev() {
                    if let Some(child_element) = ElementRef::wrap(child) {
                        stack.push((child_element.clone(), tmp.clone()));
                    } else if child.value().is_text() {
                        let text = child.value().as_text().unwrap().to_string().trim().to_string();
                        if text == "" {
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
