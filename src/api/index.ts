/**
 * Tauri API 调用的统一入口文件
 * 导出所有 API 模块的函数和类型
 */

// 导出所有类型定义
export type * from './types'

// 导出插件相关 API
export {
  scanPlugins,
  mountPlugin,
  disposePlugin,
  connectPlugin,
  disconnectPlugin,
  getPluginStatus,
  getCurrentInstance,
  sendMessageToPlugin
} from './plugins'

// 导出插件UI相关 API
export {
  getPluginUi,
  handlePluginUiEvent,
  listenPluginUiUpdate
} from './plugin-ui'

// 导出常用的 Tauri API（重新导出以便统一管理）
export { invoke } from '@tauri-apps/api/core'
