import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { usePluginStore } from './plugins'

export interface PluginMessage {
  id: string
  content: string
  timestamp: Date
  type: 'sent' | 'received'
  pluginId: string
}

export interface PluginMessages {
  [pluginId: string]: PluginMessage[]
}

const STORAGE_KEY = 'chat-client-messages'

export const useMessageStore = defineStore('messages', () => {
  // 状态
  const messagesByPlugin = ref<PluginMessages>({})
  const isLoading = ref(false)

  // 获取插件 store 实例
  const pluginStore = usePluginStore()

  // 事件监听器
  const eventListeners = ref<UnlistenFn[]>([])

  // 计算属性 - 当前插件的消息
  const currentMessages = computed(() => {
    const currentPluginId = pluginStore.currentPluginId
    if (!currentPluginId) return []
    return messagesByPlugin.value[currentPluginId] || []
  })

  // 生成消息ID
  const generateMessageId = () => {
    return `msg_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`
  }

  // 添加消息
  const addMessage = (content: string, type: 'sent' | 'received', pluginId?: string) => {
    const targetPluginId = pluginId || pluginStore.currentPluginId
    if (!targetPluginId) {
      console.warn('无法添加消息：没有指定插件ID且当前没有活跃插件')
      return
    }

    const message: PluginMessage = {
      id: generateMessageId(),
      content,
      timestamp: new Date(),
      type,
      pluginId: targetPluginId
    }

    // 确保插件消息数组存在
    if (!messagesByPlugin.value[targetPluginId]) {
      messagesByPlugin.value[targetPluginId] = []
    }

    messagesByPlugin.value[targetPluginId].push(message)
    saveToStorage()
    
    return message
  }

  // 清空指定插件的消息
  const clearPluginMessages = (pluginId?: string) => {
    const targetPluginId = pluginId || pluginStore.currentPluginId
    if (!targetPluginId) {
      console.warn('无法清空消息：没有指定插件ID且当前没有活跃插件')
      return
    }

    messagesByPlugin.value[targetPluginId] = []
    saveToStorage()
  }

  // 清空所有消息
  const clearAllMessages = () => {
    messagesByPlugin.value = {}
    saveToStorage()
  }

  // 获取指定插件的消息
  const getPluginMessages = (pluginId: string): PluginMessage[] => {
    return messagesByPlugin.value[pluginId] || []
  }

  // 获取所有插件的消息数量统计
  const getMessageCounts = computed(() => {
    const counts: Record<string, number> = {}
    for (const [pluginId, messages] of Object.entries(messagesByPlugin.value)) {
      counts[pluginId] = messages.length
    }
    return counts
  })

  // 保存到本地存储
  const saveToStorage = () => {
    try {
      const data = {
        messagesByPlugin: messagesByPlugin.value,
        timestamp: Date.now()
      }
      localStorage.setItem(STORAGE_KEY, JSON.stringify(data))
    } catch (error) {
      console.error('保存消息到本地存储失败:', error)
    }
  }

  // 从本地存储加载
  const loadFromStorage = () => {
    try {
      isLoading.value = true
      const stored = localStorage.getItem(STORAGE_KEY)
      if (stored) {
        const data = JSON.parse(stored)
        if (data.messagesByPlugin) {
          // 恢复消息数据，确保时间戳是 Date 对象
          const restoredMessages: PluginMessages = {}
          for (const [pluginId, messages] of Object.entries(data.messagesByPlugin)) {
            restoredMessages[pluginId] = (messages as any[]).map(msg => ({
              ...msg,
              timestamp: new Date(msg.timestamp)
            }))
          }
          messagesByPlugin.value = restoredMessages
        }
      }
    } catch (error) {
      console.error('从本地存储加载消息失败:', error)
    } finally {
      isLoading.value = false
    }
  }

  // 发送消息（集成插件通信）
  const sendMessage = async (content: string) => {
    const currentPluginId = pluginStore.currentPluginId
    if (!currentPluginId) {
      throw new Error('没有活跃的插件')
    }

    // 记录发送的消息
    addMessage(content, 'sent', currentPluginId)

    try {
      // 调用插件发送消息
      const response = await pluginStore.sendMessage(content)
      
      return response
    } catch (error) {
      // 如果发送失败，可以选择是否保留发送记录
      // 这里选择保留，但可以添加失败标记
      throw error
    }
  }

  // 设置事件监听器
  const setupEventListeners = async () => {
    try {
      // 监听插件挂载事件
      const unlistenMount = await listen('plugin-mounted', (event) => {
        console.log('Plugin mounted event:', event.payload)
        try {
          const data = JSON.parse(event.payload as string)
          console.log(data)
          addMessage(`插件 ${data.plugin} 已挂载`, 'received', data.plugin)
        } catch (e) {
          console.error('Failed to parse plugin-mounted event:', e)
        }
      })
      eventListeners.value.push(unlistenMount)

      // 监听插件卸载事件
      const unlistenDispose = await listen('plugin-disposed', (event) => {
        console.log('Plugin disposed event:', event.payload)
        try {
          const data = JSON.parse(event.payload as string)
          addMessage(`插件 ${data.plugin} 已卸载`, 'received', data.plugin)
        } catch (e) {
          console.error('Failed to parse plugin-disposed event:', e)
        }
      })
      eventListeners.value.push(unlistenDispose)

      // 监听插件连接事件
      const unlistenConnect = await listen('plugin-connected', (event) => {
        console.log('Plugin connected event:', event.payload)
        try {
          const data = JSON.parse(event.payload as string)
          addMessage(`插件 ${data.plugin} 已连接`, 'received', data.plugin)
        } catch (e) {
          console.error('Failed to parse plugin-connected event:', e)
        }
      })
      eventListeners.value.push(unlistenConnect)

      // 监听插件断开连接事件
      const unlistenDisconnect = await listen('plugin-disconnected', (event) => {
        console.log('Plugin disconnected event:', event.payload)
        try {
          const data = JSON.parse(event.payload as string)
          addMessage(`插件 ${data.plugin} 已断开连接`, 'received', data.plugin)
        } catch (e) {
          console.error('Failed to parse plugin-disconnected event:', e)
        }
      })
      eventListeners.value.push(unlistenDisconnect)

      // 监听插件消息接收事件
      const unlistenMessageReceived = await listen('plugin-message-received', (event) => {
        console.log('Plugin message received event:', event.payload)
        try {
          const data = JSON.parse(event.payload as string)
          console.log(`[${data.plugin}] 收到消息: ${data.message}`, 'received', data.plugin)
        } catch (e) {
          console.error('Failed to parse plugin-message-received event:', e)
        }
      })
      eventListeners.value.push(unlistenMessageReceived)

      // 监听插件消息响应事件
      const unlistenMessageResponse = await listen('plugin-message-response', (event) => {
        try {
          const data = JSON.parse(event.payload as string)
          addMessage(data.response, 'received', data.plugin)
        } catch (e) {
          console.error('Failed to parse plugin-message-response event:', e)
        }
      })
      eventListeners.value.push(unlistenMessageResponse)
    } catch (error) {
      console.error('Failed to setup event listeners:', error)
    }
  }

  // 清理事件监听器
  const cleanupEventListeners = () => {
    eventListeners.value.forEach(unlisten => {
      try {
        unlisten()
      } catch (error) {
        console.error('Failed to unlisten event:', error)
      }
    })
    eventListeners.value = []
  }

  // 初始化时加载数据和设置事件监听器
  loadFromStorage()
  setupEventListeners()

  // 调试信息
  console.log('消息 Store 已初始化')

  return {
    // 状态
    messagesByPlugin,
    isLoading,

    // 计算属性
    currentMessages,
    getMessageCounts,

    // 方法
    addMessage,
    clearPluginMessages,
    clearAllMessages,
    getPluginMessages,
    sendMessage,
    loadFromStorage,
    saveToStorage,
    cleanupEventListeners
  }
})
