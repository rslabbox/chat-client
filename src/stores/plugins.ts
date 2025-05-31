import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { ElMessage } from 'element-plus'
import {
  scanPlugins,
  mountPlugin,
  disposePlugin,
  connectPlugin,
  disconnectPlugin,
  getPluginStatus,
  getCurrentPlugin,
  sendMessageToPlugin,
  type PluginMetadata
} from '@/api'

export interface PluginState {
  isMounted: boolean
  isConnected: boolean
  isLoading: boolean
  error?: string
}

export const usePluginStore = defineStore('plugins', () => {
  // 状态
  const plugins = ref<PluginMetadata[]>([])
  const currentPluginId = ref<string | null>(null)
  const pluginStates = ref<Record<string, PluginState>>({})
  const isLoading = ref(false)

  // 计算属性
  const currentPlugin = computed(() => {
    if (!currentPluginId.value) return null
    return plugins.value.find(p => p.id === currentPluginId.value) || null
  })

  const isCurrentPluginConnected = computed(() => {
    if (!currentPluginId.value) return false
    return pluginStates.value[currentPluginId.value]?.isConnected || false
  })

  const isCurrentPluginMounted = computed(() => {
    if (!currentPluginId.value) return false
    return pluginStates.value[currentPluginId.value]?.isMounted || false
  })

  // 获取插件状态
  const getPluginState = (pluginId: string): PluginState => {
    return pluginStates.value[pluginId] || {
      isMounted: false,
      isConnected: false,
      isLoading: false
    }
  }

  // 设置插件状态
  const setPluginState = (pluginId: string, state: Partial<PluginState>) => {
    pluginStates.value[pluginId] = {
      ...getPluginState(pluginId),
      ...state
    }
  }

  // 加载插件列表
  const loadPlugins = async () => {
    try {
      isLoading.value = true
      plugins.value = await scanPlugins()
      
      // 获取当前活跃插件
      const current = await getCurrentPlugin()
      currentPluginId.value = current || null
      
      // 更新所有插件状态
      for (const plugin of plugins.value) {
        const status = await getPluginStatus(plugin.id)
        if (status) {
          setPluginState(plugin.id, {
            isMounted: status[0],
            isConnected: status[1],
            isLoading: false
          })
        }
      }
    } catch (error) {
      console.error('Failed to load plugins:', error)
      ElMessage.error('加载插件列表失败')
    } finally {
      isLoading.value = false
    }
  }

  // 挂载插件
  const mountPluginById = async (pluginId: string) => {
    try {
      setPluginState(pluginId, { isLoading: true, error: undefined })
      
      const result = await mountPlugin(pluginId)
      
      // 更新状态
      currentPluginId.value = pluginId
      setPluginState(pluginId, {
        isMounted: true,
        isConnected: false,
        isLoading: false
      })
      
      // 清除其他插件的挂载状态
      for (const plugin of plugins.value) {
        if (plugin.id !== pluginId) {
          setPluginState(plugin.id, {
            isMounted: false,
            isConnected: false,
            isLoading: false
          })
        }
      }
      
      ElMessage.success(result)
      return true
    } catch (error) {
      const errorMsg = error as string
      setPluginState(pluginId, { 
        isLoading: false, 
        error: errorMsg 
      })
      ElMessage.error(`插件挂载失败: ${errorMsg}`)
      return false
    }
  }

  // 卸载插件
  const disposePluginById = async (pluginId: string) => {
    try {
      setPluginState(pluginId, { isLoading: true, error: undefined })
      
      const result = await disposePlugin(pluginId)
      
      // 更新状态
      if (currentPluginId.value === pluginId) {
        currentPluginId.value = null
      }
      setPluginState(pluginId, {
        isMounted: false,
        isConnected: false,
        isLoading: false
      })
      
      ElMessage.success(result)
      return true
    } catch (error) {
      const errorMsg = error as string
      setPluginState(pluginId, { 
        isLoading: false, 
        error: errorMsg 
      })
      ElMessage.error(`插件卸载失败: ${errorMsg}`)
      return false
    }
  }

  // 连接插件
  const connectPluginById = async (pluginId: string) => {
    try {
      setPluginState(pluginId, { isLoading: true, error: undefined })
      
      const result = await connectPlugin(pluginId)
      
      setPluginState(pluginId, {
        isConnected: true,
        isLoading: false
      })
      
      ElMessage.success(result)
      return true
    } catch (error) {
      const errorMsg = error as string
      setPluginState(pluginId, { 
        isLoading: false, 
        error: errorMsg 
      })
      ElMessage.error(`插件连接失败: ${errorMsg}`)
      return false
    }
  }

  // 断开插件连接
  const disconnectPluginById = async (pluginId: string) => {
    try {
      setPluginState(pluginId, { isLoading: true, error: undefined })
      
      const result = await disconnectPlugin(pluginId)
      
      setPluginState(pluginId, {
        isConnected: false,
        isLoading: false
      })
      
      ElMessage.success(result)
      return true
    } catch (error) {
      const errorMsg = error as string
      setPluginState(pluginId, { 
        isLoading: false, 
        error: errorMsg 
      })
      ElMessage.error(`插件断开连接失败: ${errorMsg}`)
      return false
    }
  }

  // 切换插件（挂载新插件，自动卸载旧插件）
  const switchToPlugin = async (pluginId: string) => {
    return await mountPluginById(pluginId)
  }

  // 向当前插件发送消息
  const sendMessage = async (message: string) => {
    try {
      if (!currentPluginId.value) {
        throw new Error('没有活跃的插件')
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
    currentPluginId,
    pluginStates,
    isLoading,
    
    // 计算属性
    currentPlugin,
    isCurrentPluginConnected,
    isCurrentPluginMounted,
    
    // 方法
    getPluginState,
    loadPlugins,
    mountPluginById,
    disposePluginById,
    connectPluginById,
    disconnectPluginById,
    switchToPlugin,
    sendMessage
  }
})
