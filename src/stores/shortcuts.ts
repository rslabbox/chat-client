import { defineStore } from 'pinia'
import { ref } from 'vue'

// 快捷短语接口
export interface Shortcut {
  id: string
  title: string
  content: string
  createdAt: Date
  updatedAt: Date
}

// 插件快捷短语数据结构
export interface PluginShortcuts {
  shortcuts: Shortcut[]
}

// 快捷短语管理器数据结构
export interface ShortcutsManager {
  [pluginId: string]: PluginShortcuts
}

const SHORTCUTS_STORAGE_KEY = 'chat-client-shortcuts-manager'

export const useShortcutsStore = defineStore('shortcuts', () => {
  // 状态
  const shortcutsManager = ref<ShortcutsManager>({})

  // 生成快捷短语ID
  const generateShortcutId = (): string => {
    return `shortcut_${Date.now()}_${Math.random().toString(36).substring(2, 11)}`
  }

  // 确保插件快捷短语数据存在
  const ensurePluginShortcuts = (pluginId: string): void => {
    if (!shortcutsManager.value[pluginId]) {
      shortcutsManager.value[pluginId] = {
        shortcuts: []
      }
    }
  }

  // 根据 pluginId 获取快捷短语列表
  const getShortcutsByPluginId = (pluginId: string): Shortcut[] => {
    ensurePluginShortcuts(pluginId)
    return shortcutsManager.value[pluginId].shortcuts
  }

  // 根据 shortcutId 和 pluginId 获取单个快捷短语
  const getShortcutById = (shortcutId: string, pluginId: string): Shortcut | null => {
    ensurePluginShortcuts(pluginId)
    const shortcuts = shortcutsManager.value[pluginId].shortcuts
    return shortcuts.find(shortcut => shortcut.id === shortcutId) || null
  }

  // 添加快捷短语
  const addShortcut = (
    pluginId: string,
    title: string,
    content: string
  ): Shortcut => {
    ensurePluginShortcuts(pluginId)
    
    const shortcut: Shortcut = {
      id: generateShortcutId(),
      title: title.trim(),
      content: content.trim(),
      createdAt: new Date(),
      updatedAt: new Date()
    }

    shortcutsManager.value[pluginId].shortcuts.unshift(shortcut)
    saveToStorage()
    
    return shortcut
  }

  // 更新快捷短语
  const updateShortcut = (
    shortcutId: string,
    pluginId: string,
    title: string,
    content: string
  ): boolean => {
    ensurePluginShortcuts(pluginId)
    
    const shortcuts = shortcutsManager.value[pluginId].shortcuts
    const shortcutIndex = shortcuts.findIndex(s => s.id === shortcutId)
    
    if (shortcutIndex === -1) {
      return false
    }

    shortcuts[shortcutIndex] = {
      ...shortcuts[shortcutIndex],
      title: title.trim(),
      content: content.trim(),
      updatedAt: new Date()
    }

    saveToStorage()
    return true
  }

  // 删除快捷短语
  const deleteShortcut = (shortcutId: string, pluginId: string): boolean => {
    ensurePluginShortcuts(pluginId)
    
    const shortcuts = shortcutsManager.value[pluginId].shortcuts
    const shortcutIndex = shortcuts.findIndex(s => s.id === shortcutId)
    
    if (shortcutIndex === -1) {
      return false
    }

    shortcuts.splice(shortcutIndex, 1)
    saveToStorage()
    return true
  }

  // 清空指定插件的所有快捷短语
  const clearPluginShortcuts = (pluginId: string): number => {
    ensurePluginShortcuts(pluginId)
    
    const shortcuts = shortcutsManager.value[pluginId].shortcuts
    const deletedCount = shortcuts.length
    
    shortcutsManager.value[pluginId].shortcuts = []
    
    if (deletedCount > 0) {
      saveToStorage()
    }
    
    return deletedCount
  }

  // 获取所有插件的快捷短语统计
  const getShortcutsStats = () => {
    const stats: Record<string, { shortcutCount: number }> = {}
    
    Object.keys(shortcutsManager.value).forEach(pluginId => {
      stats[pluginId] = {
        shortcutCount: shortcutsManager.value[pluginId].shortcuts.length
      }
    })
    
    return stats
  }

  // 获取所有快捷短语（按更新时间排序）
  const getAllShortcuts = (): Array<Shortcut & { pluginId: string }> => {
    const allShortcuts: Array<Shortcut & { pluginId: string }> = []
    
    Object.keys(shortcutsManager.value).forEach(pluginId => {
      shortcutsManager.value[pluginId].shortcuts.forEach(shortcut => {
        allShortcuts.push({
          ...shortcut,
          pluginId
        })
      })
    })
    
    return allShortcuts.sort((a, b) => b.updatedAt.getTime() - a.updatedAt.getTime())
  }

  // 保存到本地存储
  const saveToStorage = (): void => {
    try {
      const data = {
        shortcutsManager: shortcutsManager.value,
        timestamp: Date.now()
      }
      localStorage.setItem(SHORTCUTS_STORAGE_KEY, JSON.stringify(data))
    } catch (error) {
      console.error('保存快捷短语到本地存储失败:', error)
    }
  }

  // 从本地存储加载
  const loadFromStorage = (): void => {
    try {
      const stored = localStorage.getItem(SHORTCUTS_STORAGE_KEY)
      if (stored) {
        const data = JSON.parse(stored)
        if (data.shortcutsManager) {
          // 恢复数据，确保时间戳是 Date 对象
          const restoredManager: ShortcutsManager = {}
          
          Object.keys(data.shortcutsManager).forEach(pluginId => {
            restoredManager[pluginId] = {
              shortcuts: data.shortcutsManager[pluginId].shortcuts.map((shortcut: any) => ({
                ...shortcut,
                createdAt: new Date(shortcut.createdAt),
                updatedAt: new Date(shortcut.updatedAt)
              }))
            }
          })
          
          shortcutsManager.value = restoredManager
        }
      }
    } catch (error) {
      console.error('从本地存储加载快捷短语失败:', error)
      shortcutsManager.value = {}
    }
  }

  // 清空所有快捷短语
  const clearAllShortcuts = (): void => {
    shortcutsManager.value = {}
    saveToStorage()
  }

  // 初始化时加载数据
  loadFromStorage()

  return {
    // 状态
    shortcutsManager,

    // 基础方法
    generateShortcutId,

    // 查询方法
    getShortcutsByPluginId,
    getShortcutById,
    getAllShortcuts,
    getShortcutsStats,

    // 快捷短语操作方法
    addShortcut,
    updateShortcut,
    deleteShortcut,
    clearPluginShortcuts,
    clearAllShortcuts,

    // 存储方法
    saveToStorage,
    loadFromStorage
  }
})
