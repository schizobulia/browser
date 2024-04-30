use crate::qaq::ChangeText;
use bean::css::CSSRule;
use bean::{css::CSSStyleSheet, node::Node};
use bevy::prelude::{Color, Query, Style, Text};

pub fn change_text_action(query: &mut Query<(&mut Text, &mut Style)>, change_text: ChangeText) {
    let text = query.get_mut(change_text.id);
    match text {
        Ok(mut nodes) => match nodes.0.sections.get_mut(0) {
            Some(node) => {
                node.value = change_text.value.clone();
            }
            None => {}
        },
        Err(err) => {
            println!("err: {:?}", err);
        }
    }
}

/**
 * 选择器查询
 */
pub fn change_style_action(
    style: CSSStyleSheet,
    list: &mut Vec<Node>,
    query: &mut Query<(&mut Text, &mut Style)>,
) {
    let mut queue = list.iter_mut().collect::<Vec<_>>();
    let rules = style.rules;
    while !queue.is_empty() {
        let node = queue.remove(0);
        for rule in rules.iter() {
            if node
                .clone()
                .attributes
                .iter()
                .any(|(key, value)| key == "id" && value == &rule.selector[1..])
            {
                if rule.val.len() != 0 {
                    change_dom_style(query, node, rule.clone());
                }
            }
        }
        queue.extend(node.children.iter_mut().collect::<Vec<_>>());
    }
}

fn change_dom_style(query: &mut Query<(&mut Text, &mut Style)>, node: &mut Node, rules: CSSRule) {
    match node.clone().text {
        Some(dom_text) => match query.get_mut(dom_text.id.unwrap()) {
            Ok(nodes) => {
                let mut text = nodes.0;
                for rule in rules.val.clone() {
                    let mut tag = true;
                    match node.style_sheet_list.clone() {
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
