/**
 * 插件相关的 Tauri API 调用
 */

import { invoke } from '@tauri-apps/api/core'
import type { PluginMetadata } from './types'

/**
 * 扫描并获取所有可用的插件列表
 * @returns Promise<PluginMetadata[]> 插件列表
 */
export async function scanPlugins(): Promise<PluginMetadata[]> {
  try {
    const plugins = await invoke<PluginMetadata[]>('scan_plugins')
    return plugins
  } catch (error) {
    console.error('Failed to scan plugins:', error)
    // 如果调用失败，返回默认数据
    return [
      {
        id: 'default_plugin',
        disabled: false,
        name: '默认插件',
        description: '默认插件描述',
        version: '1.0.0'
      }
    ]
  }
}
