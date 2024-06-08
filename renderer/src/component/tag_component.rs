use crate::component::input;
use crate::css::{get_color_by_style, get_element_unit, parse_css};
use bean::css::{CSSStyleSheet, CssVal, SourceType};
use bean::dom_component::DomComponent;
use bevy::{
    render::color::Color,
    text::TextStyle,
    ui::{node_bundles::NodeBundle, FlexDirection, Style, Val},
    utils::default,
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct HTMLTagComponent {
    tag_name: String,
    pub style: Style,
    styl_sheet: CSSStyleSheet,
    attributes: HashMap<String, String>,
    style_text_inner: TextStyle,
    bundle: NodeBundle,
    dom: DomComponent,
}

impl HTMLTagComponent {
    pub fn new(tag_name: String, attributes: HashMap<String, String>) -> Self {
        Self {
            tag_name,
            attributes,
            style: Style {
                ..Default::default()
            },
            styl_sheet: CSSStyleSheet::new(),
            style_text_inner: TextStyle {
                color: Color::BLACK,
                ..default()
            },
            bundle: NodeBundle { ..default() },
            dom: DomComponent {
                id: None,
                tag_name: "".to_string(),
            },
        }
    }

    pub fn init(&mut self) {
        let t = self.block_inline();
        if t == "block" {
            self.style.flex_direction = FlexDirection::Column;
        }
        self.dom = DomComponent {
            id: None,
            tag_name: self.tag_name.clone(),
        };
        self.init_built_component();
        self.init_css();
        self.init_style();
        self.bundle.style = self.style.clone();
    }

    fn init_built_component(&mut self) {
        if self.tag_name == "input" {
            input::init_style(
                &mut self.bundle,
                &mut self.style,
                &mut self.style_text_inner,
            );
        }
    }

    fn init_style(&mut self) {
        let style_val = self.attributes.get("style");
        match style_val {
            Some(style) => {
                for item in parse_css(format!("{}{}{}", "{", style, "}"), SourceType::Inline).rules
                {
                    self.styl_sheet.rules.push(item.clone());
                    for i in item.val {
                        set_style(i.0, i.1, &mut self.style_text_inner, &mut self.style);
                    }
                }
            }
            None => {}
        }
    }

    fn init_css(&mut self) {
        let name = &self.tag_name;
        if name == "div" {
            self.style.margin = bevy::ui::UiRect {
                top: Val::Px(0.5),
                bottom: Val::Px(0.5),
                ..default()
            };
        }
    }
    pub fn block_inline(&mut self) -> &str {
        let tag = !vec!["div", "p", "html", "body"].contains(&self.tag_name.as_str());
        if tag {
            return "inline";
        } else {
            return "block";
        }
    }

    pub fn get_attributes(&self) -> HashMap<String, String> {
        self.attributes.clone()
    }

    pub fn get_style_sheet(&self) -> CSSStyleSheet {
        self.styl_sheet.clone()
    }

    pub fn get_bundle(&self) -> NodeBundle {
        self.bundle.clone()
    }

    pub fn get_style_text_inner(&self) -> TextStyle {
        self.style_text_inner.clone()
    }

    pub fn get_dom(&self) -> DomComponent {
        self.dom.clone()
    }
}

fn set_style(key: String, val: CssVal, style_text_inner: &mut TextStyle, style: &mut Style) {
    match key.as_str() {
        "width" => {
            style.width = get_element_unit(val.value, val.unit);
        }
        "height" => {
            style.height = get_element_unit(val.value, val.unit);
        }
        "color" => {
            style_text_inner.color = get_color_by_style(val.value.trim().to_string());
        }
        "flex-direction" => {
            style.flex_direction = FlexDirection::Column;
        }
        _ => {}
    };
}
