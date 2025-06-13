import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { usePluginStore } from './plugins'
import { useHistoryStore } from './history'
import { useTabManagerStore } from './tabManager'

// 页面状态接口
export interface PageState {
  pluginId: string
  instanceId: string
  sessionId: string | null
  title?: string
  createdAt: Date
  updatedAt: Date
}

// 页面历史记录接口
export interface PageHistory {
  id: string
  pageState: PageState
  timestamp: Date
}

export const usePageManagerStore = defineStore('pageManager', () => {
  // 状态
  const currentPage = ref<PageState | null>(null)
  const pageHistory = ref<PageHistory[]>([])
  const isLoading = ref(false)

  // 获取其他store实例
  const pluginStore = usePluginStore()
  const historyStore = useHistoryStore()
  const tabManagerStore = useTabManagerStore()

  // 计算属性
  const currentPluginId = computed(() => currentPage.value?.pluginId || null)
  const currentInstanceId = computed(() => currentPage.value?.instanceId || null)
  const currentSessionId = computed(() => currentPage.value?.sessionId || null)

  // 当前页面是否有效
  const isCurrentPageValid = computed(() => {
    return !!(currentPage.value?.pluginId && currentPage.value?.instanceId && currentPage.value?.sessionId)
  })

  // 生成页面ID
  const generatePageId = () => {
    return `page_${Date.now()}_${Math.random().toString(36).substring(2, 11)}`
  }

  // 生成实例ID
  const generateInstanceId = () => {
    return crypto.randomUUID()
  }

  // 创建新页面
  const createNewPage = async (pluginId: string, title?: string) => {
    try {
      isLoading.value = true

      // 生成新的实例ID
      const instanceId = generateInstanceId()

      // 挂载插件实例
      await pluginStore.mountPluginById(pluginId, instanceId)

      // 创建新会话
      const sessionId = historyStore.createNewSession(pluginId, title)
      if (!sessionId) {
        throw new Error('创建会话失败')
      }

      // 创建页面状态
      const pageState: PageState = {
        pluginId,
        instanceId,
        sessionId: sessionId,
        title: title || `新页面 ${new Date().toLocaleString()}`,
        createdAt: new Date(),
        updatedAt: new Date()
      }

      // 更新当前页面
      currentPage.value = pageState

      // 添加到历史记录
      addToHistory(pageState)

      return pageState
    } catch (error) {
      console.error('创建新页面失败:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  // 切换到指定页面
  const switchToPage = async (pageState: PageState) => {
    try {
      isLoading.value = true

      // 切换到已存在的插件实例（不重新挂载）
      await pluginStore.switchToExistingInstance(pageState.pluginId, pageState.instanceId)

      // 切换会话
      const sessionSwitched = pageState.sessionId ? historyStore.getSessionById(pageState.sessionId) : []
      if (!sessionSwitched) {
        console.warn('会话切换失败，可能会话不存在')
      }

      // 更新当前页面
      currentPage.value = {
        ...pageState,
        updatedAt: new Date()
      }

      // 更新历史记录
      updateHistoryTimestamp(pageState)

      return true
    } catch (error) {
      console.error('切换页面失败:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  // 切换到指定会话（在当前插件实例下）
  const switchToSession = async (sessionId: string) => {
    try {
      if (!currentPage.value) {
        throw new Error('当前没有活跃页面')
      }

      // 切换会话
      const sessionSwitched = historyStore.getSessionById(sessionId)
      if (!sessionSwitched) {
        throw new Error('会话切换失败')
      }

      // 更新当前页面的会话ID
      currentPage.value = {
        ...currentPage.value,
        sessionId,
        updatedAt: new Date()
      }

      // 同步更新当前活跃标签页的sessionId
      const tabUpdated = tabManagerStore.updateCurrentTabSession(sessionId)
      if (!tabUpdated) {
        console.warn('标签页sessionId同步失败，但页面会话切换成功')
      }

      return true
    } catch (error) {
      console.error('切换会话失败:', error)
      throw error
    }
  }

  // 在当前页面创建新会话（不重新挂载实例）
  const createNewSessionInCurrentPage = async (title?: string): Promise<string | null> => {
    try {
      if (!currentPage.value) {
        throw new Error('当前没有活跃页面')
      }

      // 创建新会话
      const sessionId = historyStore.createNewSession(currentPage.value.pluginId, title)
      if (!sessionId) {
        throw new Error('创建会话失败')
      }

      // 更新当前页面的会话ID
      currentPage.value = {
        ...currentPage.value,
        sessionId,
        updatedAt: new Date()
      }

      // 同步更新当前活跃标签页的sessionId
      const tabUpdated = tabManagerStore.updateCurrentTabSession(sessionId)
      if (!tabUpdated) {
        console.warn('标签页sessionId同步失败，但页面会话创建成功')
      }

      // 添加到历史记录
      addToHistory(currentPage.value)

      return sessionId
    } catch (error) {
      console.error('创建新会话失败:', error)
      throw error
    }
  }

  // 添加到历史记录
  const addToHistory = (pageState: PageState) => {
    const historyItem: PageHistory = {
      id: generatePageId(),
      pageState: { ...pageState },
      timestamp: new Date()
    }

    // 检查是否已存在相同的页面状态
    const existingIndex = pageHistory.value.findIndex(
      item =>
        item.pageState.pluginId === pageState.pluginId &&
        item.pageState.instanceId === pageState.instanceId &&
        item.pageState.sessionId === pageState.sessionId
    )

    if (existingIndex >= 0) {
      // 更新现有记录的时间戳
      pageHistory.value[existingIndex].timestamp = new Date()
    } else {
      // 添加新记录
      pageHistory.value.unshift(historyItem)

      // 限制历史记录数量（保留最近50条）
      if (pageHistory.value.length > 50) {
        pageHistory.value = pageHistory.value.slice(0, 50)
      }
    }

    saveHistoryToStorage()
  }

  // 更新历史记录时间戳
  const updateHistoryTimestamp = (pageState: PageState) => {
    const existingIndex = pageHistory.value.findIndex(
      item =>
        item.pageState.pluginId === pageState.pluginId &&
        item.pageState.instanceId === pageState.instanceId &&
        item.pageState.sessionId === pageState.sessionId
    )

    if (existingIndex >= 0) {
      pageHistory.value[existingIndex].timestamp = new Date()
      // 移动到最前面
      const item = pageHistory.value.splice(existingIndex, 1)[0]
      pageHistory.value.unshift(item)
      saveHistoryToStorage()
    }
  }

  // 获取页面历史记录
  const getPageHistory = () => {
    return pageHistory.value.slice().sort((a, b) => b.timestamp.getTime() - a.timestamp.getTime())
  }

  // 获取指定插件的页面历史
  const getPluginPageHistory = (pluginId: string) => {
    return pageHistory.value
      .filter(item => item.pageState.pluginId === pluginId)
      .sort((a, b) => b.timestamp.getTime() - a.timestamp.getTime())
  }

  // 删除页面历史记录
  const removeFromHistory = (historyId: string) => {
    const index = pageHistory.value.findIndex(item => item.id === historyId)
    if (index >= 0) {
      pageHistory.value.splice(index, 1)
      saveHistoryToStorage()
      return true
    }
    return false
  }

  // 清空历史记录
  const clearHistory = () => {
    pageHistory.value = []
    saveHistoryToStorage()
  }

  // 保存历史记录到本地存储
  const saveHistoryToStorage = () => {
    try {
      const historyData = pageHistory.value.map(item => ({
        ...item,
        pageState: {
          ...item.pageState,
          createdAt: item.pageState.createdAt.toISOString(),
          updatedAt: item.pageState.updatedAt.toISOString()
        },
        timestamp: item.timestamp.toISOString()
      }))
      localStorage.setItem('page-manager-history', JSON.stringify(historyData))
    } catch (error) {
      console.error('保存页面历史记录失败:', error)
    }
  }

  // 从本地存储加载历史记录
  const loadHistoryFromStorage = () => {
    try {
      const stored = localStorage.getItem('page-manager-history')
      if (stored) {
        const historyData = JSON.parse(stored)
        pageHistory.value = historyData.map((item: any) => ({
          ...item,
          pageState: {
            ...item.pageState,
            createdAt: new Date(item.pageState.createdAt),
            updatedAt: new Date(item.pageState.updatedAt)
          },
          timestamp: new Date(item.timestamp)
        }))
      }
    } catch (error) {
      console.error('加载页面历史记录失败:', error)
      pageHistory.value = []
    }
  }

  // 初始化时加载历史记录
  loadHistoryFromStorage()

  return {
    // 状态
    currentPage,
    pageHistory,
    isLoading,

    // 计算属性
    currentPluginId,
    currentInstanceId,
    currentSessionId,
    isCurrentPageValid,

    // 方法
    createNewPage,
    switchToPage,
    switchToSession,
    createNewSessionInCurrentPage,
    getPageHistory,
    getPluginPageHistory,
    removeFromHistory,
    clearHistory,
    loadHistoryFromStorage,
    saveHistoryToStorage
  }
})
