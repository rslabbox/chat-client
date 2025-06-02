/**
 * Plugin UI 相关的 TypeScript 类型定义
 */

// UI组件类型枚举
export type ComponentType = 
  | {
      type: 'Button'
      label: string
      icon?: string
      enabled: boolean
    }
  | {
      type: 'TextField'
      hint: string
      value: string
    }

// 单个UI组件
export interface Component {
  id: string
  component: ComponentType
}

// 插件UI容器
export type PluginUi = Component[]

// UI事件类型
export interface PluginUiEvent {
  plugin: string
  componentId: string
  value: string
}

// UI更新事件
export interface PluginUiUpdateEvent {
  plugin: string
}
