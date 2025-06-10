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
 * 挂载插件实例
 * @param pluginId 插件ID
 * @param instanceId 可选的实例ID，如果不提供则自动生成
 * @returns Promise<string> 成功消息
 */
export async function mountPlugin(pluginId: string, instanceId: string): Promise<string> {
  return await invoke<string>('mount_plugin', { pluginId, instanceId })
}

/**
 * 卸载插件实例
 * @param instanceId 实例ID
 * @returns Promise<string> 成功消息
 */
export async function disposePlugin(instanceId: string): Promise<string> {
  return await invoke<string>('dispose_plugin', { instanceId })
}

/**
 * 连接插件实例
 * @param instanceId 实例ID
 * @returns Promise<string> 成功消息
 */
export async function connectPlugin(instanceId: string): Promise<string> {
  return await invoke<string>('connect_plugin', { instanceId })
}

/**
 * 断开插件实例连接
 * @param instanceId 实例ID
 * @returns Promise<string> 成功消息
 */
export async function disconnectPlugin(instanceId: string): Promise<string> {
  return await invoke<string>('disconnect_plugin', { instanceId })
}

/**
 * 获取插件实例状态
 * @param instanceId 实例ID
 * @returns Promise<[boolean, boolean] | null> [是否挂载, 是否连接] 或 null
 */
export async function getPluginStatus(instanceId: string): Promise<[boolean, boolean] | null> {
  return await invoke<[boolean, boolean] | null>('get_plugin_status', { instanceId })
}

/**
 * 获取当前活跃插件实例ID
 * @returns Promise<string | null> 当前实例ID 或 null
 */
export async function getCurrentInstance(): Promise<string | null> {
  return await invoke<string | null>('get_current_instance')
}

/**
 * 向当前插件发送消息
 * @param message 要发送的消息
 * @returns Promise<string> 插件返回的响应
 */
export async function sendMessageToPlugin(message: string): Promise<string> {
  return await invoke<string>('send_message_to_plugin', { message })
}
