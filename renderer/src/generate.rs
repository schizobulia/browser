use bean::css::{CSSRule, SourceType};
use bevy::{prelude::*};
use bevy_egui::egui::TextBuffer;
use scraper::ElementRef;
use std::collections::HashMap;
use crate::css::{conversion_style, conversion_text_style};

pub enum NodeResult {
    Script(String),
    Style(String),
    Component(NodeBundle, CSSRule, TextStyle, HashMap<String, String>),
}

pub fn get_node_result(element: ElementRef, tag: String) -> NodeResult {
    let mut attributes = HashMap::new();
    element.value().attrs.clone().iter().for_each(|attr| {
        attributes.insert(attr.0.local.to_string(), attr.1.to_string());
    });
    if tag == "script" {
        let mut script = String::new();
        for child in element.children() {
            script.push_str(child.value().as_text().unwrap());
        }
        return NodeResult::Script(script);
    }
    if tag == "style" {
        let mut style: String = String::new();
        for child in element.children() {
            style.push_str(child.value().as_text().unwrap());
        }
        return NodeResult::Style(style);
    }

    let style_val = attributes.get("style");
    let mut styl_sheet = CSSRule::new();
    let mut style_inner = Style {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        ..default()
    };
    let mut style_text_inner = TextStyle {
        color: Color::BLACK,
        ..default()
    };
    match style_val {
        Some(style) => {
            let style_arr = style.split(";");
            style_arr.for_each(|item| {
                let item_arr = item.split(":").collect::<Vec<&str>>();
                match item_arr.get(0) {
                    Some(key) => match item_arr.get(1) {
                        Some(val) => {
                            styl_sheet.val.insert(key.to_string(), val.to_string());
                            if key.as_str() == "width" {
                                conversion_style(
                                    key.to_string(),
                                    val.to_string(),
                                    &mut style_inner,
                                );
                            }
                            if key.as_str() == "color" {
                                conversion_text_style(
                                    key.to_string(),
                                    val.trim().to_string(),
                                    &mut style_text_inner,
                                );
                            }
                        }
                        None => {}
                    },
                    None => {}
                }
                styl_sheet.selector = String::new();
                styl_sheet.source = SourceType::Inline;
            });
        }
        None => {}
    }

    if tag == "div" {
        let bundle: NodeBundle = NodeBundle {
            style: Style { ..style_inner },
            ..default()
        };
        return NodeResult::Component(bundle, styl_sheet, style_text_inner, attributes);
    }

    if tag == "p" {
        let bundle: NodeBundle = NodeBundle {
            style: Style {
                // width: Val::Percent(100.0),
                margin: UiRect {
                    top: Val::Percent(0.5),
                    bottom: Val::Percent(0.5),
                    ..default()
                },
                flex_direction: FlexDirection::Column,
                ..style_inner
            },
            ..default()
        };
        return NodeResult::Component(bundle, styl_sheet, style_text_inner, attributes);
    }

    if tag == "html" {
        let bundle: NodeBundle = NodeBundle {
            style: Style { ..style_inner },
            ..default()
        };
        return NodeResult::Component(bundle, styl_sheet, style_text_inner, attributes);
    }

    if tag == "body" {
        let bundle: NodeBundle = NodeBundle {
            style: Style { ..style_inner },
            ..default()
        };
        return NodeResult::Component(bundle, styl_sheet, style_text_inner, attributes);
    }

    let bundle: NodeBundle = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..style_inner
        },
        ..default()
    };
    return NodeResult::Component(bundle, styl_sheet, style_text_inner, attributes);
}
