/**
 * Plugin UI 相关的 TypeScript 类型定义
 */

// UI组件类型枚举
export type ComponentType =
  | {
      type: 'Label'
      text: string
    }
  | {
      type: 'Button'
      text: string
      enabled: boolean
    }
  | {
      type: 'TextEdit'
      value: string
      hint: string
    }
  | {
      type: 'ComboBox'
      options: string[]
      selected: number | null
      placeholder: string
    }
  | {
      type: 'Toggle'
      value: boolean
    }
  | {
      type: 'Horizontal'
      children: Component[]
    }
  | {
      type: 'TextField'
      hint: string
      value: string
    }
  | {
      type: 'Text'
      value: string
    }
  | {
      type: 'Select'
      options: string[]
      selected?: string
    }
  | {
      type: 'Container'
      layout: ContainerLayout
      children: Component[]
    }

// 容器布局类型
export type ContainerLayout =
  | 'Horizontal'
  | 'Vertical'
  | { Grid: { columns: number } }

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
