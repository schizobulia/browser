use std::collections::HashMap;
use crate::css::{conversion_style, conversion_text_style};
use bean::css::{CSSRule, SourceType};
use bevy::{
    render::color::Color, text::TextStyle, ui::{node_bundles::NodeBundle, FlexDirection, Style, Val}, utils::default
};

#[derive(Clone)]
pub struct HTMLTagComponent {
    tag_name: String,
    pub style: Style,
    styl_sheet: CSSRule,
    attributes: HashMap<String, String>,
    style_text_inner: TextStyle,
    bundle: NodeBundle,
}

impl HTMLTagComponent {
    pub fn new(tag_name: String, attributes: HashMap<String, String>) -> Self {
        Self {
            tag_name,
            attributes,
            style: Style {
                ..Default::default()
            },
            styl_sheet: CSSRule::new(),
            style_text_inner: TextStyle {
                color: Color::BLACK,
                ..default()
            },
            bundle: NodeBundle {
                ..default()
            },
        }
    }

    pub fn init(&mut self) {
        let mut style_inner = Style {
            ..Default::default()
        };
        let t = self.block_inline();
        if t == "block" {
            style_inner.width = Val::Percent(100.0);
            style_inner.flex_direction = FlexDirection::Column;
        }
        self.init_css(&mut style_inner);
        self.init_style(&mut style_inner);
        self.bundle = NodeBundle {
            style: style_inner,
            ..default()
        };
    }

    fn init_style(&mut self, style_inner: &mut Style) {
        let style_val = self.attributes.get("style");
        match style_val {
            Some(style) => {
                let style_arr = style.split(";");
                style_arr.for_each(|item| {
                    let item_arr = item.split(":").collect::<Vec<&str>>();
                    match item_arr.get(0) {
                        Some(key) => match item_arr.get(1) {
                            Some(val) => {
                                self.styl_sheet.val.insert(key.to_string(), val.to_string());
                                if key.to_string() == "width" {
                                    conversion_style(
                                        key.to_string(),
                                        val.to_string(),
                                        style_inner,
                                    );
                                }
                                if key.to_string() == "color" {
                                    conversion_text_style(
                                        key.to_string(),
                                        val.trim().to_string(),
                                        &mut self.style_text_inner,
                                    );
                                }
                            }
                            None => {}
                        },
                        None => {}
                    }
                    self.styl_sheet.selector = String::new();
                    self.styl_sheet.source = SourceType::Inline;
                });
            }
            None => {}
        }
    }

    fn init_css(&mut self, style_inner: &mut Style) {
        let name = &self.tag_name;
        if name == "div" {
            style_inner.margin = bevy::ui::UiRect {
                top: Val::Percent(0.5),
                bottom: Val::Percent(0.5),
                ..default()
            };
        }
    }
    fn block_inline(&mut self) -> &str {
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

    pub fn get_style_sheet(&self) -> CSSRule {
        self.styl_sheet.clone()
    }

    pub fn get_bundle(&self) -> NodeBundle {
        self.bundle.clone()
    }

    pub fn get_style_text_inner(&self) -> TextStyle {
        self.style_text_inner.clone()
    }
}
