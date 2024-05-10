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
    pub val: HashMap<String, String>,
    pub css_text: String,
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
