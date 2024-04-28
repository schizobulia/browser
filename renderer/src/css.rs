use std::collections::HashMap;

use bevy::render::color::Color;
use bevy::text::TextStyle;
use bevy::ui::{Style, Val, FlexDirection};
use bevy::utils::default;
use cssparser::*;
use bean::css::{StyleSheet, SourceType};


pub fn parse_css(css: String) {
    let mut input = ParserInput::new(css.as_str());
    let mut parser = Parser::new(&mut input);
    let mut selector = String::new();
    let mut css_property_value = StyleSheet::new();
    while let Ok(token) = parser.next_including_whitespace_and_comments() {
        let _ = match token {
            Token::Function(_) | Token::ParenthesisBlock => Token::CloseParenthesis,
            Token::SquareBracketBlock => Token::CloseSquareBracket,
            Token::CurlyBracketBlock => {
                css_property_value.selector = selector.clone().trim().to_string();
                css_property_value.source = SourceType::StyleTag;
                css_property_value.val = HashMap::new();
                selector = String::new();
                Token::CloseCurlyBracket
            },
            _ => {
                selector.push_str(token.to_css_string().as_str());
                continue
            }
        };
        let _ = parser.parse_nested_block(|input| -> Result<_, ParseError<()>> {
            let mut key = String::new();
            while let Ok(token) = input.next_including_whitespace_and_comments() {
                match token {
                    Token::Ident(ident) => {
                        if key.is_empty() {
                            key = ident.to_string();
                        } else {
                            css_property_value.val.insert(key.clone(), ident.to_string());
                        }
                    },
                    Token::Colon => {
                        continue;
                    },
                  
                    Token::Semicolon => {
                        key = String::new();
                        continue;
                    },
                    Token::Dimension { has_sign, value, int_value, unit } => {
                        let _ = int_value;
                        css_property_value.val.insert(key.clone(), format!("{}{}{}", if has_sign.clone() { "" } else { "" }, value, unit));
                    }
                    _ => {
                        continue;
                    }   
                }
            }
            Ok(())
        });

        println!("{:?}", css_property_value);
    }
}


pub fn conversion_style(key: String, value: String, style: &mut Style){
    match key.as_str() {
        "width" => {
            style.width = Val::Percent(value.parse::<f32>().unwrap());
        },
        "flex-direction" => {
            style.flex_direction = FlexDirection::Column;
        },
        _ => {
            default()
        }
    }
}

pub fn conversion_text_style(key: String, value: String, style: &mut TextStyle) {
    match key.as_str() {
        "color" => {
            match value.as_str() {
                "blank" => {
                    style.color = Color::BLACK;
                },
                "blue" => {
                    style.color = Color::BLUE;
                },
                "red" => {
                    style.color = Color::RED;
                },
                "yellow" => {
                    style.color = Color::YELLOW;
                },
                _ => {
                    style.color = Color::BLACK;
                }
            }
        },
        _ => {
            default()
        }
    }
    
}