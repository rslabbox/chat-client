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
  messageType?: 'normal' | 'success' | 'warning' | 'error' | 'info'
  isStreaming?: boolean
  streamId?: string
}

export interface StreamMessage {
  id: string
  streamId: string
  pluginId: string
  sessionId: string
  content: string
  timestamp: Date
  status: 'active' | 'paused' | 'completed' | 'error' | 'cancelled'
  streamType?: string
  metadata?: string
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

export interface StreamMessages {
  [streamId: string]: StreamMessage
}

const STORAGE_KEY = 'chat-client-messages'
const SESSIONS_STORAGE_KEY = 'chat-client-sessions'

export const useMessageStore = defineStore('messages', () => {
  // 状态
  const messagesByPlugin = ref<PluginMessages>({})
  const chatSessions = ref<ChatSessions>({})
  const streamMessages = ref<StreamMessages>({})
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

    console.log(messagesByPlugin.value)

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
  const addMessage = (content: string, type: 'sent' | 'received', pluginId?: string, messageType?: 'normal' | 'success' | 'warning' | 'error' | 'info') => {
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
      sessionId,
      messageType: messageType || 'normal'
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

  // 流式消息处理方法
  const handleStreamStart = (message: any) => {
    const { plugin_id, data } = message
    const { stream_id, stream_type, metadata } = data
    const sessionId = currentSessionId.value
    if (!sessionId) return

    const streamMessage: StreamMessage = {
      id: generateMessageId(),
      streamId: stream_id,
      pluginId: plugin_id,
      sessionId,
      content: '',
      timestamp: new Date(),
      status: 'active',
      streamType: stream_type,
      metadata
    }

    streamMessages.value[stream_id] = streamMessage

    // 在消息列表中添加一个占位消息
    const placeholderMessage: PluginMessage = {
      id: generateMessageId(),
      content: '',
      timestamp: new Date(),
      type: 'received',
      pluginId: plugin_id,
      sessionId,
      messageType: 'info',
      isStreaming: true,
      streamId: stream_id
    }

    if (!messagesByPlugin.value[plugin_id]) {
      messagesByPlugin.value[plugin_id] = []
    }
    messagesByPlugin.value[plugin_id].push(placeholderMessage)
    saveToStorage()
  }

  const handleStreamData = (message: any) => {
    const { data } = message
    const { stream_id, chunk, is_final } = data
    const streamMessage = streamMessages.value[stream_id]
    if (!streamMessage) return

    // 更新流消息内容
    streamMessage.content += chunk
    streamMessage.timestamp = new Date()

    if (is_final) {
      streamMessage.status = 'completed'
    }

    // 更新对应的占位消息
    const pluginMessages = messagesByPlugin.value[streamMessage.pluginId] || []
    const placeholderIndex = pluginMessages.findIndex(msg => msg.streamId === stream_id)
    if (placeholderIndex !== -1) {
      pluginMessages[placeholderIndex].content = streamMessage.content
      if (is_final) {
        pluginMessages[placeholderIndex].isStreaming = false
      }
    }

    saveToStorage()
  }

  const handleStreamEnd = (message: any) => {
    const { data } = message
    const { stream_id, success, error } = data
    const streamMessage = streamMessages.value[stream_id]
    if (!streamMessage) return

    streamMessage.status = success ? 'completed' : 'error'
    streamMessage.timestamp = new Date()

    // 更新对应的占位消息
    const pluginMessages = messagesByPlugin.value[streamMessage.pluginId] || []
    const placeholderIndex = pluginMessages.findIndex(msg => msg.streamId === stream_id)
    if (placeholderIndex !== -1) {
      pluginMessages[placeholderIndex].isStreaming = false
      if (!success && error) {
        pluginMessages[placeholderIndex].content += `\n[错误: ${error}]`
        pluginMessages[placeholderIndex].messageType = 'error'
      }
    }

    saveToStorage()
  }

  const handleStreamControl = (message: any, action: 'pause' | 'resume' | 'cancel') => {
    const { data } = message
    const { stream_id } = data
    const streamMessage = streamMessages.value[stream_id]
    if (!streamMessage) return

    switch (action) {
      case 'pause':
        streamMessage.status = 'paused'
        break
      case 'resume':
        streamMessage.status = 'active'
        break
      case 'cancel':
        streamMessage.status = 'cancelled'
        // 更新对应的占位消息
        const pluginMessages = messagesByPlugin.value[streamMessage.pluginId] || []
        const placeholderIndex = pluginMessages.findIndex(msg => msg.streamId === stream_id)
        if (placeholderIndex !== -1) {
          pluginMessages[placeholderIndex].isStreaming = false
          pluginMessages[placeholderIndex].content += '\n[已取消]'
          pluginMessages[placeholderIndex].messageType = 'warning'
        }
        break
    }

    streamMessage.timestamp = new Date()
    saveToStorage()
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

      // 监听插件消息响应事件（旧协议兼容）
      const unlistenMessageResponse = await listen('plugin-message-response', (event) => {
        try {
          const data = JSON.parse(event.payload as string)
          addMessage(data.response, 'received', data.plugin)
        } catch (e) {
          console.error('Failed to parse plugin-message-response event:', e)
        }
      })
      eventListeners.value.push(unlistenMessageResponse)

      // 监听新的插件消息事件
      const unlistenPluginMessage = await listen('plugin-message', (event) => {
        console.log('Plugin message event:', event.payload)
        try {
          const data = JSON.parse(event.payload as string)
          addMessage(data.content, 'received', data.plugin_id, data.message_type)
        } catch (e) {
          console.error('Failed to parse plugin-message event:', e)
        }
      })
      eventListeners.value.push(unlistenPluginMessage)

      // 监听流式消息事件
      const unlistenPluginStream = await listen('plugin-stream', (event) => {
        try {
          const data = JSON.parse(event.payload as string)
          switch (data.type) {
            case 'stream_start':
              handleStreamStart(data)
              break
            case 'stream_data':
              handleStreamData(data)
              break
            case 'stream_end':
              handleStreamEnd(data)
              break
            case 'stream_pause':
              handleStreamControl(data, 'pause')
              break
            case 'stream_resume':
              handleStreamControl(data, 'resume')
              break
            case 'stream_cancel':
              handleStreamControl(data, 'cancel')
              break
            default:
              console.warn('Unknown stream message type:', data.type)
          }
        } catch (e) {
          console.error('Failed to parse plugin-stream event:', e)
        }
      })
      eventListeners.value.push(unlistenPluginStream)
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
    streamMessages,
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

    // 流式消息方法
    handleStreamStart,
    handleStreamData,
    handleStreamEnd,
    handleStreamControl,

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
