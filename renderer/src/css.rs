use std::collections::HashMap;

use bean::css::{CSSRule, CSSStyleSheet, SourceType};
use bevy::render::color::Color;
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
        "width" => match value.parse::<f32>() {
            Ok(val) => {
                style.width = Val::Percent(val);
            }
            Err(err) => {
                println!("err conversion_style {:?}", err);
            }
        },
        "flex-direction" => {
            style.flex_direction = FlexDirection::Column;
        }
        _ => default(),
    }
}

/**
 * Conversion color
 */
pub fn get_color_by_style(value: String) -> Color {
    if value.as_str().starts_with("#") {
        match Color::hex(value) {
            Ok(v) => return v,
            Err(_) => Color::BLACK,
        }
    } else {
        match value.as_str() {
            "alice_blue" => Color::rgb(0.94, 0.97, 1.0),
            "antique_white" => Color::rgb(0.98, 0.92, 0.84),
            "aquamarine" => Color::rgb(0.49, 1.0, 0.83),
            "azure" => Color::rgb(0.94, 1.0, 1.0),
            "beige" => Color::rgb(0.96, 0.96, 0.86),
            "bisque" => Color::rgb(1.0, 0.89, 0.77),
            "black" => Color::rgb(0.0, 0.0, 0.0),
            "blue" => Color::rgb(0.0, 0.0, 1.0),
            "crimson" => Color::rgb(0.86, 0.08, 0.24),
            "cyan" => Color::rgb(0.0, 1.0, 1.0),
            "dark_gray" => Color::rgb(0.25, 0.25, 0.25),
            "dark_green" => Color::rgb(0.0, 0.5, 0.0),
            "fuchsia" => Color::rgb(1.0, 0.0, 1.0),
            "gold" => Color::rgb(1.0, 0.84, 0.0),
            "gray" => Color::rgb(0.5, 0.5, 0.5),
            "green" => Color::rgb(0.0, 1.0, 0.0),
            "indigo" => Color::rgb(0.29, 0.0, 0.51),
            "lime_green" => Color::rgb(0.2, 0.8, 0.2),
            "maroon" => Color::rgb(0.5, 0.0, 0.0),
            "midnight_blue" => Color::rgb(0.1, 0.1, 0.44),
            "navy" => Color::rgb(0.0, 0.0, 0.5),
            "none" => Color::rgba(0.0, 0.0, 0.0, 0.0),
            "olive" => Color::rgb(0.5, 0.5, 0.0),
            "orange" => Color::rgb(1.0, 0.65, 0.0),
            "orange_red" => Color::rgb(1.0, 0.27, 0.0),
            "pink" => Color::rgb(1.0, 0.08, 0.58),
            "purple" => Color::rgb(0.5, 0.0, 0.5),
            "red" => Color::rgb(1.0, 0.0, 0.0),
            "salmon" => Color::rgb(0.98, 0.5, 0.45),
            "sea_green" => Color::rgb(0.18, 0.55, 0.34),
            "silver" => Color::rgb(0.75, 0.75, 0.75),
            "teal" => Color::rgb(0.0, 0.5, 0.5),
            "tomato" => Color::rgb(1.0, 0.39, 0.28),
            "turquoise" => Color::rgb(0.25, 0.88, 0.82),
            "violet" => Color::rgb(0.93, 0.51, 0.93),
            "white" => Color::rgb(1.0, 1.0, 1.0),
            "yellow" => Color::rgb(1.0, 1.0, 0.0),
            "yellow_green" => Color::rgb(0.6, 0.8, 0.2),
            _ => Color::BLACK,
        }
    }
}

fn rgba_to_hex(r: f32, g: f32, b: f32, a: f32) -> String {
    let r = (r * 255.0).round() as u8;
    let g = (g * 255.0).round() as u8;
    let b = (b * 255.0).round() as u8;
    let a = (a * 255.0).round() as u8;

    format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
}

/**
 * Convert color name to hex
 */
fn color_to_hex(color_name: &str) -> String {
    let color: Color = get_color_by_style(color_name.to_string());
    rgba_to_hex(color.r(), color.g(), color.b(), color.a())
}
