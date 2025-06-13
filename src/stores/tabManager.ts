import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { usePageManagerStore } from './pageManager'
import { useHistoryStore } from './history'
import { usePluginStore } from './plugins'

// 标签页接口
export interface Tab {
  id: string                    // 标签页唯一ID
  title: string                 // 标签页显示标题
  pluginId: string             // 插件ID
  instanceId: string           // 插件实例ID（每个标签页独立实例）
  sessionId: string            // 会话ID（对应history中的session）
  isActive: boolean            // 是否为当前活跃标签页
  isPinned: boolean            // 是否固定标签页
  createdAt: Date             // 创建时间
  updatedAt: Date             // 最后更新时间
  icon?: string               // 标签页图标
  color?: string              // 标签页颜色标识
}

// 插件会话统计
export interface PluginSessionStats {
  pluginId: string
  totalSessions: number
  totalMessages: number
  lastActivity: Date
  activeSessions: string[]    // 当前打开的会话ID列表
}

const TAB_STORAGE_KEY = 'chat-client-tabs'
const MAX_TABS = 20 // 最大标签页数量限制

export const useTabManagerStore = defineStore('tabManager', () => {
  // 状态
  const tabs = ref<Tab[]>([])
  const activeTabId = ref<string | null>(null)
  const nextTabIndex = ref(1)
  const isLoading = ref(false)

  // 获取其他store实例
  const pageManagerStore = usePageManagerStore()
  const historyStore = useHistoryStore()
  const pluginStore = usePluginStore()

  // 计算属性
  const activeTabs = computed(() => tabs.value.filter(tab => !tab.isPinned))
  const pinnedTabs = computed(() => tabs.value.filter(tab => tab.isPinned))
  const activeTab = computed(() => tabs.value.find(tab => tab.id === activeTabId.value) || null)
  const tabCount = computed(() => tabs.value.length)
  const canCreateNewTab = computed(() => tabCount.value < MAX_TABS)

  // 生成标签页ID
  const generateTabId = (): string => {
    return `tab_${Date.now()}_${Math.random().toString(36).substring(2, 11)}`
  }

  // 生成实例ID
  const generateInstanceId = (): string => {
    return `instance_${Date.now()}_${Math.random().toString(36).substring(2, 11)}`
  }

  // 创建新标签页
  const createNewTab = async (pluginId: string, title?: string): Promise<Tab | null> => {
    try {
      if (!canCreateNewTab.value) {
        throw new Error(`标签页数量已达到最大限制 ${MAX_TABS}`)
      }

      isLoading.value = true

      // 生成新的实例ID和标签页ID
      const instanceId = generateInstanceId()
      const tabId = generateTabId()

      // 挂载插件实例
      await pluginStore.mountPluginById(pluginId, instanceId)

      // 创建新会话
      const sessionId = historyStore.createNewSession(pluginId, title)
      if (!sessionId) {
        throw new Error('创建会话失败')
      }

      // 获取插件信息用于生成标题
      const plugin = pluginStore.plugins.find(p => p.id === pluginId)
      const defaultTitle = title || `${plugin?.name || '插件'} ${nextTabIndex.value}`

      // 创建标签页
      const newTab: Tab = {
        id: tabId,
        title: defaultTitle,
        pluginId,
        instanceId,
        sessionId,
        isActive: false, // 先设为false，稍后激活
        isPinned: false,
        createdAt: new Date(),
        updatedAt: new Date(),
        icon: plugin?.icon,
        color: plugin?.color
      }

      // 添加到标签页列表
      tabs.value.push(newTab)
      nextTabIndex.value++

      // 激活新标签页
      await switchToTab(tabId)

      // 保存到存储
      saveToStorage()

      return newTab
    } catch (error) {
      console.error('创建标签页失败:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  // 切换到指定标签页
  const switchToTab = async (tabId: string): Promise<boolean> => {
    try {
      const tab = tabs.value.find(t => t.id === tabId)
      if (!tab) {
        throw new Error(`标签页 ${tabId} 不存在`)
      }

      isLoading.value = true

      // 取消当前活跃标签页
      if (activeTabId.value) {
        const currentTab = tabs.value.find(t => t.id === activeTabId.value)
        if (currentTab) {
          currentTab.isActive = false
        }
      }

      // 激活新标签页
      tab.isActive = true
      tab.updatedAt = new Date()
      activeTabId.value = tabId

      // 通过pageManager切换到对应的页面状态
      const pageState = {
        pluginId: tab.pluginId,
        instanceId: tab.instanceId,
        sessionId: tab.sessionId,
        title: tab.title,
        createdAt: tab.createdAt,
        updatedAt: tab.updatedAt
      }

      await pageManagerStore.switchToPage(pageState)

      // 保存到存储
      saveToStorage()

      return true
    } catch (error) {
      console.error('切换标签页失败:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  // 关闭标签页
  const closeTab = async (tabId: string): Promise<boolean> => {
    try {
      const tabIndex = tabs.value.findIndex(t => t.id === tabId)
      if (tabIndex === -1) {
        throw new Error(`标签页 ${tabId} 不存在`)
      }

      const tab = tabs.value[tabIndex]
      
      // 如果是固定标签页，需要确认
      if (tab.isPinned) {
        // 这里可以添加确认逻辑
        console.warn('尝试关闭固定标签页')
      }

      // 如果关闭的是当前活跃标签页，需要切换到其他标签页
      if (tab.isActive && tabs.value.length > 1) {
        // 优先切换到右侧标签页，如果没有则切换到左侧
        const nextTabIndex = tabIndex < tabs.value.length - 1 ? tabIndex + 1 : tabIndex - 1
        const nextTab = tabs.value[nextTabIndex]
        if (nextTab) {
          await switchToTab(nextTab.id)
        }
      }

      // 检查是否还有其他标签页使用同一个插件实例
      const otherTabsWithSameInstance = tabs.value.filter(t =>
        t.id !== tabId && t.instanceId === tab.instanceId
      )

      // 只有当没有其他标签页使用同一个实例时，才卸载插件实例
      if (otherTabsWithSameInstance.length === 0) {
        try {
          await pluginStore.disconnectPluginInstance(tab.instanceId)
          await pluginStore.disposePluginInstance(tab.instanceId)
          console.log(`插件实例 ${tab.instanceId} 已卸载（无其他标签页使用）`)
        } catch (error) {
          console.warn('断开插件实例失败:', error)
        }
      } else {
        console.log(`保留插件实例 ${tab.instanceId}（还有 ${otherTabsWithSameInstance.length} 个标签页在使用）`)
      }

      // 从列表中移除标签页
      tabs.value.splice(tabIndex, 1)

      // 如果没有标签页了，清空活跃标签页ID
      if (tabs.value.length === 0) {
        activeTabId.value = null
      }

      // 保存到存储
      saveToStorage()

      return true
    } catch (error) {
      console.error('关闭标签页失败:', error)
      throw error
    }
  }

  // 保存到本地存储
  const saveToStorage = () => {
    try {
      const data = {
        tabs: tabs.value,
        activeTabId: activeTabId.value,
        nextTabIndex: nextTabIndex.value
      }
      localStorage.setItem(TAB_STORAGE_KEY, JSON.stringify(data))
    } catch (error) {
      console.error('保存标签页数据失败:', error)
    }
  }

  // 重新挂载标签页对应的插件实例
  const remountPluginsFromTabs = async (): Promise<void> => {
    if (tabs.value.length === 0) {
      return
    }

    console.log(`开始重新挂载 ${tabs.value.length} 个标签页的插件实例`)

    // 收集需要挂载的插件实例（去重）
    const instancesMap = new Map<string, { pluginId: string; instanceId: string; tabIds: string[] }>()

    tabs.value.forEach(tab => {
      const key = `${tab.pluginId}_${tab.instanceId}`
      if (instancesMap.has(key)) {
        instancesMap.get(key)!.tabIds.push(tab.id)
      } else {
        instancesMap.set(key, {
          pluginId: tab.pluginId,
          instanceId: tab.instanceId,
          tabIds: [tab.id]
        })
      }
    })

    // 并行挂载所有插件实例
    const mountPromises = Array.from(instancesMap.values()).map(async ({ pluginId, instanceId, tabIds }) => {
      try {
        console.log(`重新挂载插件实例: ${pluginId} (${instanceId}), 关联标签页: ${tabIds.join(', ')}`)
        await pluginStore.switchToExistingInstance(pluginId, instanceId)
        console.log(`插件实例挂载成功: ${pluginId} (${instanceId})`)
        return { success: true, pluginId, instanceId, tabIds }
      } catch (error) {
        console.error(`插件实例挂载失败: ${pluginId} (${instanceId}):`, error)
        return { success: false, pluginId, instanceId, tabIds, error }
      }
    })

    const results = await Promise.allSettled(mountPromises)

    // 统计挂载结果
    let successCount = 0
    let failedCount = 0
    const failedTabs: string[] = []

    results.forEach((result, index) => {
      if (result.status === 'fulfilled') {
        const mountResult = result.value
        if (mountResult.success) {
          successCount++
        } else {
          failedCount++
          failedTabs.push(...mountResult.tabIds)
        }
      } else {
        failedCount++
        const instanceInfo = Array.from(instancesMap.values())[index]
        failedTabs.push(...instanceInfo.tabIds)
      }
    })

    console.log(`插件实例挂载完成: 成功 ${successCount} 个, 失败 ${failedCount} 个`)

    // 如果有失败的标签页，可以选择移除它们或标记为错误状态
    if (failedTabs.length > 0) {
      console.warn(`以下标签页的插件实例挂载失败，可能需要手动处理: ${failedTabs.join(', ')}`)
      // 这里可以选择移除失败的标签页或者保留它们但标记为错误状态
      // 暂时保留，让用户手动处理
    }
  }

  // 从本地存储加载
  const loadFromStorage = async (): Promise<void> => {
    try {
      isLoading.value = true

      const data = localStorage.getItem(TAB_STORAGE_KEY)
      if (data) {
        const parsed = JSON.parse(data)
        tabs.value = parsed.tabs || []
        activeTabId.value = parsed.activeTabId || null
        nextTabIndex.value = parsed.nextTabIndex || 1

        // 转换日期字符串为Date对象
        tabs.value.forEach(tab => {
          tab.createdAt = new Date(tab.createdAt)
          tab.updatedAt = new Date(tab.updatedAt)
        })

        // 重新挂载所有标签页对应的插件实例
        if (tabs.value.length > 0) {
          await remountPluginsFromTabs()

          // 如果有当前激活的标签页，需要同步其UI状态
          if (activeTabId.value) {
            const activeTab = tabs.value.find(tab => tab.id === activeTabId.value)
            if (activeTab) {
              console.log(`同步激活标签页的UI状态: ${activeTab.title} (${activeTab.instanceId})`)
              try {
                // 通过pageManager切换到当前激活标签页，这会触发UI同步
                const pageState = {
                  pluginId: activeTab.pluginId,
                  instanceId: activeTab.instanceId,
                  sessionId: activeTab.sessionId,
                  title: activeTab.title,
                  createdAt: activeTab.createdAt,
                  updatedAt: activeTab.updatedAt
                }
                await pageManagerStore.switchToPage(pageState)
                console.log(`激活标签页UI状态同步完成: ${activeTab.title}`)
              } catch (error) {
                console.error(`同步激活标签页UI状态失败: ${activeTab.title}:`, error)
              }
            }
          }
        }

        console.log(`从存储加载了 ${tabs.value.length} 个标签页`)
      }
    } catch (error) {
      console.error('加载标签页数据失败:', error)
    } finally {
      isLoading.value = false
    }
  }

  // 重命名标签页
  const renameTab = (tabId: string, newTitle: string): boolean => {
    const tab = tabs.value.find(t => t.id === tabId)
    if (!tab) return false

    tab.title = newTitle
    tab.updatedAt = new Date()
    saveToStorage()
    return true
  }

  // 固定/取消固定标签页
  const toggleTabPin = (tabId: string): boolean => {
    const tab = tabs.value.find(t => t.id === tabId)
    if (!tab) return false

    tab.isPinned = !tab.isPinned
    tab.updatedAt = new Date()
    saveToStorage()
    return true
  }

  // 移动标签页位置
  const moveTab = (fromIndex: number, toIndex: number): boolean => {
    if (fromIndex < 0 || fromIndex >= tabs.value.length ||
        toIndex < 0 || toIndex >= tabs.value.length) {
      return false
    }

    const tab = tabs.value.splice(fromIndex, 1)[0]
    tabs.value.splice(toIndex, 0, tab)
    saveToStorage()
    return true
  }

  // 关闭所有标签页
  const closeAllTabs = async (): Promise<void> => {
    const tabIds = [...tabs.value.map(t => t.id)]
    for (const tabId of tabIds) {
      await closeTab(tabId)
    }
  }

  // 关闭其他标签页
  const closeOtherTabs = async (keepTabId: string): Promise<void> => {
    const tabIds = tabs.value.filter(t => t.id !== keepTabId).map(t => t.id)
    for (const tabId of tabIds) {
      await closeTab(tabId)
    }
  }

  // 关闭右侧标签页
  const closeTabsToRight = async (tabId: string): Promise<void> => {
    const tabIndex = tabs.value.findIndex(t => t.id === tabId)
    if (tabIndex === -1) return

    const tabIds = tabs.value.slice(tabIndex + 1).map(t => t.id)
    for (const id of tabIds) {
      await closeTab(id)
    }
  }

  // 获取插件的会话统计
  const getPluginSessionStats = (pluginId: string): PluginSessionStats => {
    const pluginSessions = historyStore.getSessionsByPluginId(pluginId)
    const activeSessions = tabs.value
      .filter(tab => tab.pluginId === pluginId)
      .map(tab => tab.sessionId)

    const totalMessages = pluginSessions.reduce((sum, session) => sum + session.messages.length, 0)
    const lastActivity = pluginSessions.length > 0
      ? new Date(Math.max(...pluginSessions.map(s => s.updatedAt.getTime())))
      : new Date()

    return {
      pluginId,
      totalSessions: pluginSessions.length,
      totalMessages,
      lastActivity,
      activeSessions
    }
  }

  // 根据插件ID获取相关标签页
  const getTabsByPluginId = (pluginId: string): Tab[] => {
    return tabs.value.filter(tab => tab.pluginId === pluginId)
  }

  // 查找标签页
  const findTab = (predicate: (tab: Tab) => boolean): Tab | undefined => {
    return tabs.value.find(predicate)
  }

  // 根据instanceId获取对应的sessionId
  const getSessionIdByInstanceId = (instanceId: string): string | null => {
    const tab = tabs.value.find(tab => tab.instanceId === instanceId)
    return tab ? tab.sessionId : null
  }

  // 更新当前活跃标签页的sessionId
  const updateCurrentTabSession = (sessionId: string): boolean => {
    if (!activeTabId.value) {
      console.warn('没有活跃的标签页，无法更新sessionId')
      return false
    }

    const activeTab = tabs.value.find(tab => tab.id === activeTabId.value)
    if (!activeTab) {
      console.warn(`未找到活跃标签页 ${activeTabId.value}`)
      return false
    }

    // 更新标签页的sessionId和时间戳
    activeTab.sessionId = sessionId
    activeTab.updatedAt = new Date()

    // 保存到本地存储
    saveToStorage()

    console.log(`已更新标签页 ${activeTab.title} 的sessionId为: ${sessionId}`)
    return true
  }

  // 初始化函数
  const initialize = async (): Promise<void> => {
    await loadFromStorage()
  }

  // 初始化时加载数据（异步执行，不阻塞store创建）
  initialize().catch(error => {
    console.error('标签页管理器初始化失败:', error)
  })

  return {
    // 状态
    tabs,
    activeTabId,
    nextTabIndex,
    isLoading,

    // 计算属性
    activeTabs,
    pinnedTabs,
    activeTab,
    tabCount,
    canCreateNewTab,

    // 基础方法
    createNewTab,
    switchToTab,
    closeTab,
    saveToStorage,
    loadFromStorage,
    initialize,

    // 扩展方法
    renameTab,
    toggleTabPin,
    moveTab,
    closeAllTabs,
    closeOtherTabs,
    closeTabsToRight,

    // 查询方法
    getPluginSessionStats,
    getTabsByPluginId,
    findTab,
    getSessionIdByInstanceId,

    // 会话同步方法
    updateCurrentTabSession
  }
})
