/**
 * 插件相关的 Tauri API 调用
 */

import { invoke } from '@tauri-apps/api/core'
import type { PluginMetadata, AvailablePluginInfo, PluginDownloadResult } from './types'

/**
 * 扫描并获取所有可用的插件列表
 * @returns Promise<PluginMetadata[]> 插件列表
 */
export async function scanPlugins(): Promise<PluginMetadata[]> {
  console.log('扫描插件列表')
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
 * 向指定插件实例发送消息
 * @param pluginId 插件ID
 * @param instanceId 实例ID
 * @param message 要发送的消息
 * @returns Promise<string> 插件返回的响应
 */
export async function sendMessageToPlugin(
  pluginId: string,
  instanceId: string,
  message: string
): Promise<string> {
  return await invoke<string>('send_message_to_plugin', {
    pluginId,
    instanceId,
    message
  })
}

/**
 * 向当前插件发送消息（已弃用，保留向后兼容性）
 * @param message 要发送的消息
 * @returns Promise<string> 插件返回的响应
 */
export async function sendMessageToCurrentPlugin(message: string): Promise<string> {
  return await invoke<string>('send_message_to_current_plugin', { message })
}

/**
 * 扫描可用插件列表（从插件仓库）
 * @returns Promise<AvailablePluginInfo[]> 可用插件列表
 */
export async function scanAvailablePlugins(): Promise<AvailablePluginInfo[]> {
  console.log('扫描可用插件列表')
  try {
    const plugins = await invoke<AvailablePluginInfo[]>('scan_available_plugins')
    return plugins
  } catch (error) {
    console.error('Failed to scan available plugins:', error)
    throw error
  }
}

/**
 * 下载并安装插件
 * @param pluginId 插件ID
 * @returns Promise<PluginDownloadResult> 下载结果
 */
export async function downloadPlugin(pluginId: string): Promise<PluginDownloadResult> {
  console.log('下载插件:', pluginId)
  try {
    const result = await invoke<PluginDownloadResult>('download_plugin', { pluginId })
    return result
  } catch (error) {
    console.error('Failed to download plugin:', error)
    throw error
  }
}
