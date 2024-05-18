use std::collections::HashMap;

use crate::qaq::ChangeText;
use bean::css::CSSRule;
use bean::qaq;
use bean::{css::CSSStyleSheet, node::Node};
use bevy::ecs::entity::Entity;
use bevy::prelude::{Color, Query, Style, Text};
use bevy::ui::BorderColor;

/**
 * Modify the text in the node
 */
pub fn change_text_action(query: &mut Query<&mut Text>, change_text: ChangeText) {
    let text = query.get_mut(change_text.id);
    match text {
        Ok(mut node) => {
            node.sections[0].value = change_text.value.clone();
        },
        Err(err) => {
            println!("err: {:?}", err);
        }
    }
}

/**
 * Add CSSStyleSheet
 */
pub fn add_style_sheet_action(style: CSSStyleSheet, query: &mut Query<&mut Text>) {
    let node = qaq::GLOBAL_STATE.lock().unwrap();
    let rules = style.rules;
    for rule in rules.iter() {
        match node.get_node_by_tag_id(rule.selector[1..].to_string()) {
            Some(mut n) => {
                if rule.val.len() != 0 {
                    change_dom_style(query, &mut n, rule.clone());
                }
            }
            None => {}
        }
    }
    drop(node);
}

/**
 * Change the style of the node
 */
fn change_dom_style(
    query: &mut Query<&mut Text>,
    node: &mut std::sync::Arc<std::sync::Mutex<Node>>,
    rules: CSSRule,
) {
    let n = node.lock().unwrap();
    match &n.text {
        Some(dom_text) => match query.get_mut(dom_text.id.unwrap()) {
            Ok(mut text) => {
                for rule in rules.val.clone() {
                    let mut tag = true;
                    match n.style_rules.clone() {
                        Some(list) => {
                            if list.val.get(rule.0.as_str()).is_some() {
                                match list.source {
                                    bean::css::SourceType::Inline => {
                                        tag = false;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                    if tag {
                        match rule.0.as_str() {
                            "color" => match Color::hex(rule.1) {
                                Ok(color) => {
                                    text.sections[0].style.color = color;
                                }
                                _ => {}
                            },
                            "font-size" => match rule.1.parse::<f32>() {
                                Ok(size) => {
                                    text.sections[0].style.font_size = size;
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                }
            }
            Err(err) => {
                println!("err change_dom_style: {:?}", err);
            }
        },
        _ => {}
    }
}


pub fn change_stlye_action(id: Entity, css: HashMap<String, String>, query: &mut Query<(&mut Style, &mut BorderColor, Entity)>) {
    match query.get_mut(id) {
        Ok(res) => {
            let mut border_color = res.1;
            for (key, _) in css.iter()  {
                if key == "border-color" {
                    border_color.0 = Color::BLACK;
                }
            }
        },
        Err(err) => {
            println!("err change_stlye_action: {:?}", err);
        }
    }
}