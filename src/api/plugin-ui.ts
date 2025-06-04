/**
 * Plugin UI API 封装
 */

import { invoke } from '@tauri-apps/api/core'
import type { PluginUi } from './types'

/**
 * 获取插件UI定义
 * @param pluginId 插件ID
 * @returns 插件UI组件列表的JSON字符串
 */
export async function getPluginUi(pluginId: string): Promise<PluginUi> {
  const uiJson = await invoke<string>('get_plugin_ui', { pluginId })
  console.log(uiJson)
  return JSON.parse(uiJson) as PluginUi
}

/**
 * 处理插件UI事件
 * @param pluginId 插件ID
 * @param componentId 组件ID
 * @param value 事件值
 * @returns 是否处理成功
 */
export async function handlePluginUiEvent(
  pluginId: string,
  componentId: string,
  value: string
): Promise<boolean> {
  return await invoke<boolean>('handle_plugin_ui_event', {
    pluginId,
    componentId,
    value
  })
}

/**
 * 监听插件UI更新事件
 * @param callback 回调函数
 * @returns 取消监听的函数
 */
export function listenPluginUiUpdate(callback: (pluginId: string) => void) {
  // 这里需要使用Tauri的事件监听API
  // 具体实现取决于前端框架的事件系统
  console.log('Plugin UI update listener setup', callback)
  
  // 返回一个取消监听的函数
  return () => {
    console.log('Plugin UI update listener cleanup')
  }
}
