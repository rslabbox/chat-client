//! UI组件类型定义
//! 
//! 定义了插件UI系统中的各种组件类型

use serde::{Deserialize, Serialize};

/// UI组件类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ComponentType {
    Button {
        label: String,
        icon: Option<String>,
        enabled: bool,
        #[serde(skip)]
        action_id: String,
    },
    TextField {
        hint: String,
        value: String,
        #[serde(skip)]
        action_id: String,
    },
    // 可以扩展更多组件类型
}

/// 单个UI组件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub id: String,
    pub component: ComponentType,
}
