use crate::component::video::get_source_element;
use scraper::ElementRef;
use std::collections::HashMap;

use crate::component::tag_component::HTMLTagComponent;

pub enum NodeResult {
    Script(String),
    Style(String),
    Component(HTMLTagComponent),
}

fn get_script_res(element: ElementRef) -> NodeResult {
    let mut script = String::new();
    for child in element.children() {
        if let Some(val) = child.value().as_text() {
            script.push_str(val);
        }
    }
    return NodeResult::Script(script);
}

fn get_style_res(element: ElementRef) -> NodeResult {
    let mut style: String = String::new();
    for child in element.children() {
        if let Some(val) = child.value().as_text() {
            style.push_str(val);
        }
    }
    return NodeResult::Style(style);
}

/**
 * get bevy node by element
 */
pub fn get_node_result(element: ElementRef, tag: String) -> NodeResult {
    let mut attributes = HashMap::new();
    element.value().attrs.clone().iter().for_each(|attr| {
        attributes.insert(attr.0.local.to_string(), attr.1.to_string());
    });
    if tag == "script" {
        return get_script_res(element);
    }
    if tag == "style" {
        return get_style_res(element);
    }
    if tag == "video" {
        attributes.extend(get_source_element(element));
    }
    let mut tag_component: HTMLTagComponent = HTMLTagComponent::new(tag, attributes);

    tag_component.init();
    return NodeResult::Component(tag_component);
}
