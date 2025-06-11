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

// 下载响应接口
export interface DownloadResponse {
  success: boolean
  message: string
  download_path?: string
}

/**
 * 可用插件信息（来自插件仓库）
 */
export interface AvailablePluginInfo {
  id: string
  name: string
  version: string
  description: string
  author: string
  avatar?: string
  homepage?: string
  repository?: string
  license?: string
  keywords?: string[]
  min_client_version?: string
  max_client_version?: string
  platform?: string[]
  dependencies?: string[]
  download?: {
    windows?: {
      checksum: string
      download_url: string
    }
    macos?: {
      checksum: string
      download_url: string
    }
    linux?: {
      checksum: string
      download_url: string
    }
  }
}

/**
 * 插件下载结果
 */
export interface PluginDownloadResult {
  success: boolean
  message: string
  plugin_id?: string
  installed_path?: string
}

// 重新导出插件UI相关类型
export * from './plugin-ui-types'
