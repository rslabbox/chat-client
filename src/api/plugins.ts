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
    throw error
  }
}

/**
 * 挂载插件
 * @param pluginId 插件ID
 * @returns Promise<string> 成功消息
 */
export async function mountPlugin(pluginId: string): Promise<string> {
  return await invoke<string>('mount_plugin', { pluginId })
}

/**
 * 卸载插件
 * @param pluginId 插件ID
 * @returns Promise<string> 成功消息
 */
export async function disposePlugin(pluginId: string): Promise<string> {
  return await invoke<string>('dispose_plugin', { pluginId })
}

/**
 * 连接插件
 * @param pluginId 插件ID
 * @returns Promise<string> 成功消息
 */
export async function connectPlugin(pluginId: string): Promise<string> {
  return await invoke<string>('connect_plugin', { pluginId })
}

/**
 * 断开插件连接
 * @param pluginId 插件ID
 * @returns Promise<string> 成功消息
 */
export async function disconnectPlugin(pluginId: string): Promise<string> {
  return await invoke<string>('disconnect_plugin', { pluginId })
}

/**
 * 获取插件状态
 * @param pluginId 插件ID
 * @returns Promise<[boolean, boolean] | null> [是否挂载, 是否连接] 或 null
 */
export async function getPluginStatus(pluginId: string): Promise<[boolean, boolean] | null> {
  return await invoke<[boolean, boolean] | null>('get_plugin_status', { pluginId })
}

/**
 * 获取当前活跃插件
 * @returns Promise<string | null> 当前插件ID 或 null
 */
export async function getCurrentPlugin(): Promise<string | null> {
  return await invoke<string | null>('get_current_plugin')
}

/**
 * 向当前插件发送消息
 * @param message 要发送的消息
 * @returns Promise<string> 插件返回的响应
 */
export async function sendMessageToPlugin(message: string): Promise<string> {
  return await invoke<string>('send_message_to_plugin', { message })
}
