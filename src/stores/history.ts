import { defineStore } from 'pinia'
import { ref } from 'vue'

// 基础消息接口
export interface BaseMessage {
  id: string
  type: 'normal' | 'streaming'
  status: 'completed' | 'active' | 'paused' | 'error' | 'cancelled'
  content: string
  pluginId: string
  role: 'user' | 'plugin' | 'system'
  createdAt: Date
}

// 会话接口
export interface ChatSession {
  id: string
  title: string
  messages: BaseMessage[]
  pluginId: string
  createdAt: Date
  updatedAt: Date
}

// 历史消息管理器
export interface HistoryManager {
  sessions: ChatSession[]
}

const HISTORY_STORAGE_KEY = 'chat-client-history-0609'

export const useHistoryStore = defineStore('history', () => {
  // 状态
  const historyManager = ref<HistoryManager>({
    sessions: []
  })

  // 生成会话ID
  const generateSessionId = (): string => {
    return `session_${Date.now()}_${Math.random().toString(36).substring(2, 11)}`
  }

  // 生成消息ID
  const generateMessageId = (): string => {
    return `message_${Date.now()}_${Math.random().toString(36).substring(2, 11)}`
  }

  // 根据 pluginId 获取 session 列表
  const getSessionsByPluginId = (pluginId: string): ChatSession[] => {
    return historyManager.value.sessions.filter(session => session.pluginId === pluginId)
  }

  // 根据 sessionId 获取消息列表
  const getMessagesBySessionId = (sessionId: string): BaseMessage[] => {
    const session = historyManager.value.sessions.find(s => s.id === sessionId)
    return session ? session.messages : []
  }

  // 根据 sessionId 获取会话
  const getSessionById = (sessionId: string): ChatSession | null => {
    return historyManager.value.sessions.find(s => s.id === sessionId) || null
  }

  // 往 sessionId 对应的列表增加消息
  const addMessageToSession = (
    sessionId: string,
    content: string,
    messageId: string,
    role: 'user' | 'plugin' | 'system',
    type: 'normal' | 'streaming' = 'normal',
    status: 'completed' | 'active' | 'paused' | 'error' | 'cancelled' = 'completed'
  ): BaseMessage | null => {
    const sessionIndex = historyManager.value.sessions.findIndex(s => s.id === sessionId)
    if (sessionIndex === -1) {
      console.warn(`Session with id ${sessionId} not found`)
      return null
    }

    const session = historyManager.value.sessions[sessionIndex]
    const message: BaseMessage = {
      id: messageId,
      type,
      status,
      content,
      pluginId: session.pluginId,
      role,
      createdAt: new Date()
    }

    // 添加消息到会话
    session.messages.push(message)
    session.updatedAt = new Date()

    // 如果是第一条用户消息，更新会话标题
    if (session.messages.length === 1 && role === 'user') {
      session.title = content.length > 20 ? content.substring(0, 20) + '...' : content
    }

    saveToStorage()
    return message
  }

  // 更新消息状态和内容（主要用于流式消息）
  const updateMessage = (
    messageId: string,
    content: string,
    status: 'completed' | 'active' | 'paused' | 'error' | 'cancelled'
  ): boolean => {
    for (const session of historyManager.value.sessions) {
      const messageIndex = session.messages.findIndex(m => m.id === messageId)
      if (messageIndex !== -1) {
        const message = session.messages[messageIndex]
          message.content += content
          message.status = status
        
        session.updatedAt = new Date()
        saveToStorage()
        return true
      }
    }
    return false
  }

  // 创建新的消息列表（会话），并返回 sessionId
  const createNewSession = (pluginId: string, title?: string): string => {
    const sessionId = generateSessionId()
    const now = new Date()

    const session: ChatSession = {
      id: sessionId,
      title: title || `新对话 ${now.toLocaleString()}`,
      messages: [],
      pluginId,
      createdAt: now,
      updatedAt: now
    }

    historyManager.value.sessions.push(session)
    saveToStorage()

    return sessionId
  }

  // 删除整个 sessionId 列表
  const deleteSession = (sessionId: string): boolean => {
    const sessionIndex = historyManager.value.sessions.findIndex(s => s.id === sessionId)
    if (sessionIndex !== -1) {
      historyManager.value.sessions.splice(sessionIndex, 1)
      saveToStorage()
      return true
    }
    return false
  }

  // 清空指定 pluginId 下的所有 sessionId 列表
  const clearPluginSessions = (pluginId: string): number => {
    const initialLength = historyManager.value.sessions.length
    historyManager.value.sessions = historyManager.value.sessions.filter(
      session => session.pluginId !== pluginId
    )
    const deletedCount = initialLength - historyManager.value.sessions.length

    if (deletedCount > 0) {
      saveToStorage()
    }

    return deletedCount
  }

  // 清空所有会话
  const clearAllSessions = (): void => {
    historyManager.value.sessions = []
    saveToStorage()
  }

  // 保存到本地存储
  const saveToStorage = (): void => {
    try {
      const data = {
        historyManager: historyManager.value,
        timestamp: Date.now()
      }
      localStorage.setItem(HISTORY_STORAGE_KEY, JSON.stringify(data))
    } catch (error) {
      console.error('保存历史记录到本地存储失败:', error)
    }
  }

  // 从本地存储加载
  const loadFromStorage = (): void => {
    try {
      const stored = localStorage.getItem(HISTORY_STORAGE_KEY)
      if (stored) {
        const data = JSON.parse(stored)
        if (data.historyManager && data.historyManager.sessions) {
          // 恢复数据，确保时间戳是 Date 对象
          const restoredSessions: ChatSession[] = data.historyManager.sessions.map((session: any) => ({
            ...session,
            createdAt: new Date(session.createdAt),
            updatedAt: new Date(session.updatedAt),
            messages: session.messages.map((message: any) => ({
              ...message,
              createdAt: new Date(message.createdAt)
            }))
          }))

          historyManager.value = {
            sessions: restoredSessions
          }
        }
      }
    } catch (error) {
      console.error('从本地存储加载历史记录失败:', error)
      historyManager.value = { sessions: [] }
    }
  }

  // 获取所有会话（按更新时间排序）
  const getAllSessions = (): ChatSession[] => {
    return [...historyManager.value.sessions].sort((a, b) => b.updatedAt.getTime() - a.updatedAt.getTime())
  }

  // 获取会话统计信息
  const getSessionStats = () => {
    const stats: Record<string, { sessionCount: number; messageCount: number }> = {}

    historyManager.value.sessions.forEach(session => {
      if (!stats[session.pluginId]) {
        stats[session.pluginId] = { sessionCount: 0, messageCount: 0 }
      }
      stats[session.pluginId].sessionCount++
      stats[session.pluginId].messageCount += session.messages.length
    })

    return stats
  }

  // 初始化时加载数据
  loadFromStorage()

  return {
    // 状态
    historyManager,

    // 基础方法
    generateSessionId,
    generateMessageId,

    // 查询方法
    getSessionsByPluginId,
    getMessagesBySessionId,
    getSessionById,
    getAllSessions,
    getSessionStats,

    // 消息操作方法
    addMessageToSession,
    updateMessage,

    // 会话操作方法
    createNewSession,
    deleteSession,
    clearPluginSessions,
    clearAllSessions,

    // 存储方法
    saveToStorage,
    loadFromStorage
  }
})
