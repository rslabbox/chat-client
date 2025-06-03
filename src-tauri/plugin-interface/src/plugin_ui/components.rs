//! UI组件类型定义
//! 
//! 定义了插件UI系统中的各种组件类型

use serde::{Deserialize, Serialize};

/// 容器布局类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContainerLayout {
    /// 水平布局（行）
    Horizontal,
    /// 垂直布局（列）
    Vertical,
    /// 网格布局
    Grid { columns: u32 },
}

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
    Text {
        value: String,
    },
    Select {
        options: Vec<String>,
        selected: Option<String>,
        #[serde(skip)]
        action_id: String,
    },
    Container {
        layout: ContainerLayout,
        children: Vec<Component>,
    },
    // 可以扩展更多组件类型
}

/// 单个UI组件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub id: String,
    pub component: ComponentType,
}
