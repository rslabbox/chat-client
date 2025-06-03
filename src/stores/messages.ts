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
  sessionId: string
}

export interface ChatSession {
  id: string
  title: string
  pluginId: string
  createdAt: Date
  updatedAt: Date
  messageCount: number
}

export interface PluginMessages {
  [pluginId: string]: PluginMessage[]
}

export interface ChatSessions {
  [sessionId: string]: ChatSession
}

const STORAGE_KEY = 'chat-client-messages'
const SESSIONS_STORAGE_KEY = 'chat-client-sessions'

export const useMessageStore = defineStore('messages', () => {
  // 状态
  const messagesByPlugin = ref<PluginMessages>({})
  const chatSessions = ref<ChatSessions>({})
  const currentSessionId = ref<string | null>(null)
  const isLoading = ref(false)

  // 获取插件 store 实例
  const pluginStore = usePluginStore()

  // 事件监听器
  const eventListeners = ref<UnlistenFn[]>([])

  // 计算属性 - 当前会话的消息
  const currentMessages = computed(() => {
    const sessionId = currentSessionId.value
    if (!sessionId) return []

    const currentPluginId = pluginStore.currentPluginId
    if (!currentPluginId) return []

    const pluginMessages = messagesByPlugin.value[currentPluginId] || []
    return pluginMessages.filter(msg => msg.sessionId === sessionId)
  })

  // 计算属性 - 当前插件的会话列表
  const currentPluginSessions = computed(() => {
    const currentPluginId = pluginStore.currentPluginId
    if (!currentPluginId) return []

    return Object.values(chatSessions.value)
      .filter(session => session.pluginId === currentPluginId)
      .sort((a, b) => b.updatedAt.getTime() - a.updatedAt.getTime())
  })

  // 生成消息ID
  const generateMessageId = () => {
    return `msg_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`
  }

  // 生成会话ID
  const generateSessionId = () => {
    return `session_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`
  }

  // 创建新会话
  const createNewSession = (pluginId?: string, title?: string) => {
    const targetPluginId = pluginId || pluginStore.currentPluginId
    if (!targetPluginId) {
      console.warn('无法创建会话：没有指定插件ID且当前没有活跃插件')
      return null
    }

    const sessionId = generateSessionId()
    const now = new Date()

    const session: ChatSession = {
      id: sessionId,
      title: title || `新对话 ${new Date().toLocaleString()}`,
      pluginId: targetPluginId,
      createdAt: now,
      updatedAt: now,
      messageCount: 0
    }

    chatSessions.value[sessionId] = session
    currentSessionId.value = sessionId
    saveSessionsToStorage()

    return session
  }

  // 切换到指定会话
  const switchToSession = (sessionId: string) => {
    if (chatSessions.value[sessionId]) {
      currentSessionId.value = sessionId
      return true
    }
    return false
  }

  // 删除会话
  const deleteSession = (sessionId: string) => {
    if (!chatSessions.value[sessionId]) return false

    const session = chatSessions.value[sessionId]

    // 删除会话相关的消息
    const pluginMessages = messagesByPlugin.value[session.pluginId] || []
    messagesByPlugin.value[session.pluginId] = pluginMessages.filter(
      msg => msg.sessionId !== sessionId
    )

    // 删除会话
    delete chatSessions.value[sessionId]

    // 如果删除的是当前会话，切换到其他会话或创建新会话
    if (currentSessionId.value === sessionId) {
      const remainingSessions = currentPluginSessions.value
      if (remainingSessions.length > 0) {
        currentSessionId.value = remainingSessions[0].id
      } else {
        // 创建新会话
        createNewSession(session.pluginId)
      }
    }

    saveToStorage()
    saveSessionsToStorage()
    return true
  }

  // 添加消息
  const addMessage = (content: string, type: 'sent' | 'received', pluginId?: string) => {
    const targetPluginId = pluginId || pluginStore.currentPluginId
    if (!targetPluginId) {
      console.warn('无法添加消息：没有指定插件ID且当前没有活跃插件')
      return
    }

    // 确保有当前会话
    let sessionId = currentSessionId.value
    if (!sessionId) {
      const newSession = createNewSession(targetPluginId)
      sessionId = newSession?.id || null
      if (!sessionId) {
        console.warn('无法创建会话')
        return
      }
    }

    const message: PluginMessage = {
      id: generateMessageId(),
      content,
      timestamp: new Date(),
      type,
      pluginId: targetPluginId,
      sessionId
    }

    // 确保插件消息数组存在
    if (!messagesByPlugin.value[targetPluginId]) {
      messagesByPlugin.value[targetPluginId] = []
    }

    messagesByPlugin.value[targetPluginId].push(message)

    // 更新会话信息
    if (chatSessions.value[sessionId]) {
      chatSessions.value[sessionId].updatedAt = new Date()
      chatSessions.value[sessionId].messageCount += 1

      // 如果是第一条消息，用消息内容作为会话标题
      if (chatSessions.value[sessionId].messageCount === 1 && type === 'sent') {
        chatSessions.value[sessionId].title = content.length > 20
          ? content.substring(0, 20) + '...'
          : content
      }
    }

    saveToStorage()
    saveSessionsToStorage()

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

  // 保存消息到本地存储
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

  // 保存会话到本地存储
  const saveSessionsToStorage = () => {
    try {
      const data = {
        chatSessions: chatSessions.value,
        currentSessionId: currentSessionId.value,
        timestamp: Date.now()
      }
      localStorage.setItem(SESSIONS_STORAGE_KEY, JSON.stringify(data))
    } catch (error) {
      console.error('保存会话到本地存储失败:', error)
    }
  }

  // 从本地存储加载消息
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
              timestamp: new Date(msg.timestamp),
              // 为旧消息添加默认sessionId（兼容性处理）
              sessionId: msg.sessionId || 'legacy_session'
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

  // 从本地存储加载会话
  const loadSessionsFromStorage = () => {
    try {
      const stored = localStorage.getItem(SESSIONS_STORAGE_KEY)
      if (stored) {
        const data = JSON.parse(stored)
        if (data.chatSessions) {
          // 恢复会话数据，确保时间戳是 Date 对象
          const restoredSessions: ChatSessions = {}
          for (const [sessionId, session] of Object.entries(data.chatSessions)) {
            restoredSessions[sessionId] = {
              ...(session as any),
              createdAt: new Date((session as any).createdAt),
              updatedAt: new Date((session as any).updatedAt)
            }
          }
          chatSessions.value = restoredSessions
          currentSessionId.value = data.currentSessionId || null
        }
      }
    } catch (error) {
      console.error('从本地存储加载会话失败:', error)
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
  loadSessionsFromStorage()
  setupEventListeners()

  // 调试信息
  console.log('消息 Store 已初始化')

  return {
    // 状态
    messagesByPlugin,
    chatSessions,
    currentSessionId,
    isLoading,

    // 计算属性
    currentMessages,
    currentPluginSessions,
    getMessageCounts,

    // 消息方法
    addMessage,
    clearPluginMessages,
    clearAllMessages,
    getPluginMessages,
    sendMessage,
    loadFromStorage,
    saveToStorage,

    // 会话方法
    createNewSession,
    switchToSession,
    deleteSession,
    loadSessionsFromStorage,
    saveSessionsToStorage,

    // 其他方法
    cleanupEventListeners
  }
})
