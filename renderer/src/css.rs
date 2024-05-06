use std::collections::HashMap;

use bean::css::{CSSRule, CSSStyleSheet, SourceType};
use bevy::render::color::Color;
use bevy::text::TextStyle;
use bevy::ui::{FlexDirection, Style, Val};
use bevy::utils::default;
use cssparser::*;

/**
 * Parse css
 */
pub fn parse_css(css: String) -> CSSStyleSheet {
    let mut input = ParserInput::new(css.as_str());
    let mut parser = Parser::new(&mut input);
    let mut selector: String = String::new();
    let mut css_rule = CSSRule::new();
    let mut rules: Vec<CSSRule> = Vec::new();
    while let Ok(token) = parser.next_including_whitespace_and_comments() {
        let (_, r) = match token {
            Token::Function(_) | Token::ParenthesisBlock => {
                (Token::CloseParenthesis, String::new())
            }
            Token::SquareBracketBlock => (Token::CloseSquareBracket, String::new()),
            Token::CurlyBracketBlock => {
                css_rule.source = SourceType::StyleTag;
                css_rule.val = HashMap::new();
                css_rule = CSSRule::new();
                (
                    Token::CloseCurlyBracket,
                    selector.clone().trim().to_string(),
                )
            }
            _ => {
                selector.push_str(token.to_css_string().as_str());
                continue;
            }
        };
        selector = String::new();
        css_rule.selector = r.clone().trim().to_string();
        let _ = parser.parse_nested_block(|input| -> Result<_, ParseError<()>> {
            let mut key = String::new();
            while let Ok(token) = input.next_including_whitespace_and_comments() {
                match token {
                    Token::Ident(ident) => {
                        if key.is_empty() {
                            key = ident.to_string();
                        } else {
                            css_rule.val.insert(key.clone(), color_to_hex(&ident));
                        }
                    }
                    Token::Colon => {
                        continue;
                    }

                    Token::Semicolon => {
                        key = String::new();
                        continue;
                    }
                    Token::Dimension {
                        has_sign,
                        value,
                        int_value,
                        unit,
                    } => {
                        let _ = int_value;
                        let _ = unit;
                        css_rule.val.insert(
                            key.clone(),
                            format!("{}{}", if has_sign.clone() { "" } else { "" }, value),
                        );
                    }
                    _ => {
                        continue;
                    }
                }
            }
            Ok(())
        });
        rules.push(css_rule.clone());
    }
    CSSStyleSheet { rules }
}

pub fn conversion_style(key: String, value: String, style: &mut Style) {
    match key.as_str() {
        "width" => {
            style.width = Val::Percent(value.parse::<f32>().unwrap());
        }
        "flex-direction" => {
            style.flex_direction = FlexDirection::Column;
        }
        _ => default(),
    }
}

/**
 * Conversion text style
 */
pub fn conversion_text_style(key: String, value: String, style: &mut TextStyle) {
    match key.as_str() {
        "color" => match value.as_str() {
            "blank" => {
                style.color = Color::BLACK;
            }
            "blue" => {
                style.color = Color::BLUE;
            }
            "red" => {
                style.color = Color::RED;
            }
            "yellow" => {
                style.color = Color::YELLOW;
            }
            _ => {
                style.color = Color::BLACK;
            }
        },
        _ => default(),
    }
}

/**
 * Convert color name to hex
 */
fn color_to_hex(color_name: &str) -> String {
    let color_map: HashMap<&str, &str> = [
        ("aliceblue", "#f0f8ff"),
        ("antiquewhite", "#faebd7"),
        ("aqua", "#00ffff"),
        ("aquamarine", "#7fffd4"),
        ("azure", "#f0ffff"),
        ("beige", "#f5f5dc"),
        ("bisque", "#ffe4c4"),
        ("black", "#000000"),
        ("blanchedalmond", "#ffebcd"),
        ("blue", "#0000ff"),
        ("blueviolet", "#8a2be2"),
        ("brown", "#a52a2a"),
        ("burlywood", "#deb887"),
        ("cadetblue", "#5f9ea0"),
        ("white", "#ffffff"),
        ("yellow", "#ffff00"),
        ("yellowgreen", "#9acd32"),
        ("red", "#ff0000"),
    ]
    .iter()
    .cloned()
    .collect();
    match color_map.get(color_name) {
        Some(hex) => hex.to_string(),
        None => String::new(),
    }
}
