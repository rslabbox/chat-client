import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { ElMessage } from 'element-plus'
import {
  scanPlugins,
  mountPlugin,
  disposePlugin,
  connectPlugin,
  disconnectPlugin,
  getCurrentInstance,
  sendMessageToPlugin,
  type PluginMetadata
} from '@/api'

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
  const currentInstanceId = ref<string | null>(null)
  const instanceStates = ref<Record<string, PluginInstanceState>>({})
  const isLoading = ref(false)

  // 计算属性
  const currentInstance = computed(() => {
    if (!currentInstanceId.value) return null
    return instanceStates.value[currentInstanceId.value] || null
  })

  const currentPlugin = computed(() => {
    const instance = currentInstance.value
    if (!instance) return null
    return plugins.value.find(p => p.id === instance.pluginId) || null
  })

  const isCurrentInstanceConnected = computed(() => {
    return currentInstance.value?.isConnected || false
  })

  const isCurrentInstanceMounted = computed(() => {
    return currentInstance.value?.isMounted || false
  })

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
      isMounted: false,
      isConnected: false,
      isLoading: false,
      ...state
    }
  }

  // 加载插件列表
  const loadPlugins = async () => {
    try {
      isLoading.value = true
      plugins.value = await scanPlugins()

      // 获取当前活跃实例
      const currentInstance = await getCurrentInstance()
      currentInstanceId.value = currentInstance || null

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

      // 更新状态
      currentInstanceId.value = actualInstanceId
      setInstanceState(actualInstanceId, pluginId, {
        isMounted: true,
        isConnected: false,
        isLoading: false
      })

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

      // 更新状态
      if (currentInstanceId.value === instanceId) {
        currentInstanceId.value = null
      }
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

      setInstanceState(instanceId, instance.pluginId, {
        isConnected: true,
        isLoading: false
      })

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

      setInstanceState(instanceId, instance.pluginId, {
        isConnected: false,
        isLoading: false
      })

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

  // 切换到插件实例（挂载新实例，自动卸载旧实例）
  const switchToPlugin = async (pluginId: string, instanceId: string) => {
    return await mountPluginById(pluginId, instanceId)
  }

  // 向当前插件实例发送消息
  const sendMessage = async (message: string) => {
    try {
      if (!currentInstanceId.value) {
        throw new Error('没有活跃的插件实例')
      }

      const response = await sendMessageToPlugin(message)
      return response
    } catch (error) {
      const errorMsg = error as string
      ElMessage.error(`发送消息失败: ${errorMsg}`)
      throw error
    }
  }

  return {
    // 状态
    plugins,
    currentInstanceId,
    instanceStates,
    isLoading,

    // 计算属性
    currentInstance,
    currentPlugin,
    isCurrentInstanceConnected,
    isCurrentInstanceMounted,

    // 方法
    getInstanceState,
    loadPlugins,
    mountPluginById,
    disposePluginInstance,
    connectPluginInstance,
    disconnectPluginInstance,
    switchToPlugin,
    sendMessage
  }
})
