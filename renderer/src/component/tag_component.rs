use crate::component::input;
use crate::css::{conversion_style, get_color_by_style};
use bean::css::{CSSRule, SourceType};
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
    styl_sheet: CSSRule,
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
            styl_sheet: CSSRule::new(),
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
                                        &mut self.style,
                                    );
                                }
                                if key.to_string() == "color" {
                                    self.style_text_inner.color =
                                        get_color_by_style(val.trim().to_string());
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

    pub fn get_style_sheet(&self) -> CSSRule {
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
