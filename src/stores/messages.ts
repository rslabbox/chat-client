import { defineStore } from 'pinia'
import { ref, computed, watch } from 'vue'
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

  // 初始化时加载数据
  loadFromStorage()

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
    saveToStorage
  }
})
