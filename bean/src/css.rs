use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/**
 * css来源，方便做权重控制
 */
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SourceType {
    StyleTag, // style标签
    Inline, // 内联
    External, // 外部
}

/**
 * css结构,为了保证渲染时的性能
 */
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StyleSheet {
    pub source: SourceType,
    pub selector: String,
    pub val: HashMap<String, String>,
}

impl StyleSheet {
    pub fn new() -> Self {
        Self {
            source: SourceType::StyleTag,
            selector: String::new(),
            val: HashMap::new(),
        }
    }
}
