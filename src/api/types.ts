/**
 * Tauri 后端 API 相关的 TypeScript 类型定义
 */

// 插件元数据接口
export interface PluginMetadata {
  id: string
  disabled: boolean
  name: string
  description: string
  version: string
  author?: string
  library_path?: string  // 动态库文件路径
  config_path: string    // 配置文件路径
  icon?: string          // 插件图标
  color?: string         // 插件颜色
}

// 重新导出插件UI相关类型
export * from './plugin-ui-types'
