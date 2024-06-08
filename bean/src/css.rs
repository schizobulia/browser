use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/**
 * Css source, convenient for weight control
 */
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SourceType {
    StyleTag,
    Inline,
    External,
}

/**
 * https://developer.mozilla.org/zh-CN/docs/Web/API/CSSRule
 */
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CSSRule {
    pub source: SourceType,
    pub selector: String,
    pub val: HashMap<String, CssVal>,
    pub css_text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CssVal {
    pub has_sign: bool,
    pub value: String,
    pub unit: String,
}

impl CSSRule {
    pub fn new() -> Self {
        Self {
            source: SourceType::StyleTag,
            selector: String::new(),
            val: HashMap::new(),
            css_text: String::new(),
        }
    }
}

/**
 * https://developer.mozilla.org/zh-CN/docs/Web/API/CSSStyleSheet
 * */
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CSSStyleSheet {
    pub rules: Vec<CSSRule>,
}

impl CSSStyleSheet {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }
}
