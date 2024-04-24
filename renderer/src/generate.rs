use bevy::prelude::*;
use scraper::ElementRef;

pub enum NodeResult {
    Script(String),
    Style(String),
    Div(NodeBundle),
}


pub fn get_node_result(element: ElementRef) -> NodeResult {
    let tag = element.value().name().to_string();
    if tag == "script" {
        let mut script = String::new();
        for child in element.children() {
            script.push_str(child.value().as_text().unwrap());
        }
        return NodeResult::Script(script);
    } else if tag == "style" {
        let mut style: String = String::new();
        for child in element.children() {
            style.push_str(child.value().as_text().unwrap());
        }
        return NodeResult::Style(style);
    } else {
        let bundle = NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        };
        return NodeResult::Div(bundle);
    }
}
