import { defineStore } from 'pinia'
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import {
  scanPlugins,
  mountPlugin,
  disposePlugin,
  connectPlugin,
  disconnectPlugin,
  sendMessageToPlugin,
  getPluginStatus,
  type PluginMetadata
} from '@/api'
import type { BaseMessage } from './history'

export interface PluginInstanceState {
  instanceId: string
  pluginId: string
  isMounted: boolean
  isConnected: boolean
  isLoading: boolean
  error?: string
}

export const usePluginStore = defineStore('plugins', () => {
  // 状态
  const plugins = ref<PluginMetadata[]>([])
  const instanceStates = ref<Record<string, PluginInstanceState>>({})
  const isLoading = ref(false)

  // 获取实例状态
  const getInstanceState = (instanceId: string): PluginInstanceState | null => {
    return instanceStates.value[instanceId] || null
  }

  // 设置实例状态
  const setInstanceState = (instanceId: string, pluginId: string, state: Partial<Omit<PluginInstanceState, 'instanceId' | 'pluginId'>>) => {
    const existingState = instanceStates.value[instanceId]
    instanceStates.value[instanceId] = {
      ...existingState,
      instanceId,
      pluginId,
      isLoading: false,
      ...state
    }
  }

  // 从后端同步插件实例状态
  const syncInstanceState = async (instanceId: string, pluginId: string) => {
    try {
      const status = await getPluginStatus(instanceId)
      if (status) {
        const [isMounted, isConnected] = status
        setInstanceState(instanceId, pluginId, {
          isMounted,
          isConnected,
          isLoading: false,
          error: undefined
        })
        return { isMounted, isConnected }
      } else {
        // 插件实例不存在
        setInstanceState(instanceId, pluginId, {
          isMounted: false,
          isConnected: false,
          isLoading: false,
          error: '插件实例不存在'
        })
        return { isMounted: false, isConnected: false }
      }
    } catch (error) {
      const errorMsg = error as string
      setInstanceState(instanceId, pluginId, {
        isLoading: false,
        error: errorMsg
      })
      console.error('同步插件状态失败:', error)
      return null
    }
  }

  // 加载插件列表
  const loadPlugins = async () => {
    console.log('加载插件列表')
    try {
      isLoading.value = true
      plugins.value = await scanPlugins()
    } catch (error) {
      console.error('Failed to load plugins:', error)
      ElMessage.error('加载插件列表失败')
    } finally {
      isLoading.value = false
    }
  }

  // 挂载插件实例
  const mountPluginById = async (pluginId: string, instanceId: string) => {
    try {
      const tempInstanceId = instanceId;
      setInstanceState(tempInstanceId, pluginId, { isLoading: true, error: undefined })

      const result = await mountPlugin(pluginId, instanceId)

      // 从结果中提取实际的实例ID（如果后端返回了实例ID）
      const actualInstanceId = instanceId || tempInstanceId

      // 挂载成功后同步状态
      await syncInstanceState(actualInstanceId, pluginId)

      ElMessage.success(result)
      return actualInstanceId
    } catch (error) {
      const errorMsg = error as string
      const tempInstanceId = instanceId;
      setInstanceState(tempInstanceId, pluginId, {
        isLoading: false,
        error: errorMsg
      })
      ElMessage.error(`插件挂载失败: ${errorMsg}`)
      return null
    }
  }

  // 卸载插件实例
  const disposePluginInstance = async (instanceId: string) => {
    try {
      const instance = getInstanceState(instanceId)
      if (!instance) {
        throw new Error('实例不存在')
      }

      setInstanceState(instanceId, instance.pluginId, { isLoading: true, error: undefined })

      const result = await disposePlugin(instanceId)

      delete instanceStates.value[instanceId]

      ElMessage.success(result)
      return true
    } catch (error) {
      const errorMsg = error as string
      const instance = getInstanceState(instanceId)
      if (instance) {
        setInstanceState(instanceId, instance.pluginId, {
          isLoading: false,
          error: errorMsg
        })
      }
      ElMessage.error(`插件实例卸载失败: ${errorMsg}`)
      return false
    }
  }

  // 连接插件实例
  const connectPluginInstance = async (instanceId: string) => {
    try {
      const instance = getInstanceState(instanceId)
      if (!instance) {
        throw new Error('实例不存在')
      }

      setInstanceState(instanceId, instance.pluginId, { isLoading: true, error: undefined })

      const result = await connectPlugin(instanceId)

      // 连接成功后同步状态
      await syncInstanceState(instanceId, instance.pluginId)

      ElMessage.success(result)
      return true
    } catch (error) {
      const errorMsg = error as string
      const instance = getInstanceState(instanceId)
      if (instance) {
        setInstanceState(instanceId, instance.pluginId, {
          isLoading: false,
          error: errorMsg
        })
      }
      ElMessage.error(`插件实例连接失败: ${errorMsg}`)
      return false
    }
  }

  // 断开插件实例连接
  const disconnectPluginInstance = async (instanceId: string) => {
    try {
      const instance = getInstanceState(instanceId)
      if (!instance) {
        throw new Error('实例不存在')
      }

      setInstanceState(instanceId, instance.pluginId, { isLoading: true, error: undefined })

      const result = await disconnectPlugin(instanceId)

      // 断开连接成功后同步状态
      await syncInstanceState(instanceId, instance.pluginId)

      ElMessage.success(result)
      return true
    } catch (error) {
      const errorMsg = error as string
      const instance = getInstanceState(instanceId)
      if (instance) {
        setInstanceState(instanceId, instance.pluginId, {
          isLoading: false,
          error: errorMsg
        })
      }
      ElMessage.error(`插件实例断开连接失败: ${errorMsg}`)
      return false
    }
  }

  // 切换到已存在的插件实例（不重新挂载）
  const switchToExistingInstance = async (pluginId: string, instanceId: string) => {
    try {
      // 先同步状态以获取最新的插件状态
      const status = await syncInstanceState(instanceId, pluginId)

      if (!status) {
        throw new Error(`无法获取插件实例 ${instanceId} 的状态`)
      }

      // 如果实例存在但未挂载，则挂载它
      if (!status.isMounted) {
        return await mountPluginById(pluginId, instanceId)
      }

      // 实例已存在且已挂载，直接返回成功
      return instanceId
    } catch (error) {
      console.error('切换到插件实例失败:', error)
      throw error
    }
  }

  // 刷新插件列表
  const refreshPlugins = async () => {
    await loadPlugins()
  }

  // 向当前插件实例发送消息
  const sendMessage = async (message: string, pluginId: string | null, instanceId: string | null) => {
    try {
      // 如果没有提供 pluginId 和 instanceId，尝试从当前上下文获取
      let targetPluginId = pluginId
      let targetInstanceId = instanceId

      if (!targetPluginId || !targetInstanceId) {
        // 尝试从 tabManager 获取当前活跃标签页的信息
        const { useTabManagerStore } = await import('./tabManager')
        const tabManagerStore = useTabManagerStore()

        if (tabManagerStore.activeTab) {
          targetPluginId = targetPluginId || tabManagerStore.activeTab.pluginId
          targetInstanceId = targetInstanceId || tabManagerStore.activeTab.instanceId
        }

        // 如果还是没有，尝试从 pageManager 获取
        if (!targetPluginId || !targetInstanceId) {
          const { usePageManagerStore } = await import('./pageManager')
          const pageManagerStore = usePageManagerStore()

          targetPluginId = targetPluginId || pageManagerStore.currentPluginId || null
          targetInstanceId = targetInstanceId || pageManagerStore.currentInstanceId || null
        }
      }

      if (!targetPluginId || !targetInstanceId) {
        throw new Error('无法确定目标插件实例：请确保有活跃的插件标签页或提供 pluginId 和 instanceId 参数')
      }

      // 检查插件是否需要历史记录
      const plugin = plugins.value.find(p => p.id === targetPluginId)
      let history: BaseMessage[] | undefined = undefined

      if (plugin?.require_history) {
        // 如果插件需要历史记录，获取当前会话的历史记录
        try {
          const { useHistoryStore } = await import('./history')
          const historyStore = useHistoryStore()

          // 尝试获取当前会话ID
          let currentSessionId: string | null = null

          // 先从 tabManager 获取
          const { useTabManagerStore } = await import('./tabManager')
          const tabManagerStore = useTabManagerStore()
          if (tabManagerStore.activeTab?.sessionId) {
            currentSessionId = tabManagerStore.activeTab.sessionId
          }

          // 如果没有，从 pageManager 获取
          if (!currentSessionId) {
            const { usePageManagerStore } = await import('./pageManager')
            const pageManagerStore = usePageManagerStore()
            currentSessionId = pageManagerStore.currentSessionId
          }

          if (currentSessionId) {
            history = historyStore.getMessagesBySessionId(currentSessionId)
          }
        } catch (error) {
          console.warn('获取历史记录失败:', error)
          // 即使获取历史记录失败，也继续发送消息，只是不传递历史记录
        }
      }

      const response = await sendMessageToPlugin(targetPluginId, targetInstanceId, message, history)
      return response
    } catch (error) {
      const errorMsg = error as string
      ElMessage.error(`发送消息失败: ${errorMsg}`)
      throw error
    }
  }

  loadPlugins()

  return {
    // 状态
    plugins,
    instanceStates,
    isLoading,

    // 方法
    getInstanceState,
    setInstanceState,
    syncInstanceState,
    loadPlugins,
    refreshPlugins,
    mountPluginById,
    disposePluginInstance,
    connectPluginInstance,
    disconnectPluginInstance,
    switchToExistingInstance,
    sendMessage
  }
})
