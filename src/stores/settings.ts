import { ref, reactive } from 'vue'
import { defineStore } from 'pinia'

// 设置接口定义
export interface AppSettings {
  // 通用设置
  theme: 'light' | 'dark' | 'auto'
  language: 'zh-CN' | 'en-US'
  autoConnect: boolean
  
  // 插件设置
  pluginDirectory: string
  pluginHotReload: boolean
  pluginLogLevel: 'error' | 'warn' | 'info' | 'debug'
  
  // 消息设置
  messageRetentionDays: number
  maxDisplayMessages: number
  autoScrollToLatest: boolean
  
  // 高级设置
  developerMode: boolean
  hardwareAcceleration: boolean
}

// 默认设置
const defaultSettings: AppSettings = {
  // 通用设置
  theme: 'light',
  language: 'zh-CN',
  autoConnect: true,
  
  // 插件设置
  pluginDirectory: './plugins',
  pluginHotReload: false,
  pluginLogLevel: 'info',
  
  // 消息设置
  messageRetentionDays: 30,
  maxDisplayMessages: 200,
  autoScrollToLatest: true,
  
  // 高级设置
  developerMode: false,
  hardwareAcceleration: true,
}

export const useSettingsStore = defineStore('settings', () => {
  // 状态
  const settings = reactive<AppSettings>({ ...defaultSettings })
  const isLoading = ref(false)

  // UI状态
  const leftPanelVisible = ref(true)
  const rightPanelVisible = ref(false)

  // 本地存储键名
  const STORAGE_KEY = 'chat-client-settings'

  // 加载设置
  const loadSettings = async () => {
    try {
      isLoading.value = true
      
      // 从本地存储加载设置
      const savedSettings = localStorage.getItem(STORAGE_KEY)
      if (savedSettings) {
        const parsed = JSON.parse(savedSettings)
        Object.assign(settings, { ...defaultSettings, ...parsed })
      }
      
      console.log('设置加载完成:', settings)
    } catch (error) {
      console.error('加载设置失败:', error)
      // 如果加载失败，使用默认设置
      Object.assign(settings, defaultSettings)
    } finally {
      isLoading.value = false
    }
  }

  // 保存设置
  const saveSettings = async (newSettings?: Partial<AppSettings>) => {
    try {
      isLoading.value = true
      
      if (newSettings) {
        Object.assign(settings, newSettings)
      }
      
      // 保存到本地存储
      localStorage.setItem(STORAGE_KEY, JSON.stringify(settings))
      
      console.log('设置保存完成:', settings)
      return true
    } catch (error) {
      console.error('保存设置失败:', error)
      return false
    } finally {
      isLoading.value = false
    }
  }

  // 重置设置
  const resetSettings = async () => {
    try {
      isLoading.value = true
      
      // 重置为默认设置
      Object.assign(settings, defaultSettings)
      
      // 保存到本地存储
      localStorage.setItem(STORAGE_KEY, JSON.stringify(settings))
      
      console.log('设置已重置为默认值')
      return true
    } catch (error) {
      console.error('重置设置失败:', error)
      return false
    } finally {
      isLoading.value = false
    }
  }

  // 获取特定设置值
  const getSetting = <K extends keyof AppSettings>(key: K): AppSettings[K] => {
    return settings[key]
  }

  // 设置特定值
  const setSetting = async <K extends keyof AppSettings>(
    key: K, 
    value: AppSettings[K]
  ) => {
    settings[key] = value
    return await saveSettings()
  }

  // 导出设置
  const exportSettings = () => {
    return JSON.stringify(settings, null, 2)
  }

  // 导入设置
  const importSettings = async (settingsJson: string) => {
    try {
      const importedSettings = JSON.parse(settingsJson)
      
      // 验证导入的设置格式
      const validatedSettings: Partial<AppSettings> = {}
      
      // 只导入有效的设置项
      Object.keys(defaultSettings).forEach(key => {
        const settingKey = key as keyof AppSettings
        if (importedSettings.hasOwnProperty(key)) {
          validatedSettings[settingKey] = importedSettings[key]
        }
      })
      
      return await saveSettings(validatedSettings)
    } catch (error) {
      console.error('导入设置失败:', error)
      return false
    }
  }

  // 应用主题
  const applyTheme = (theme: AppSettings['theme']) => {
    // TODO: 实现主题切换逻辑
    console.log('应用主题:', theme)
    
    if (theme === 'dark') {
      document.documentElement.classList.add('dark')
    } else if (theme === 'light') {
      document.documentElement.classList.remove('dark')
    } else if (theme === 'auto') {
      // 跟随系统主题
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
      if (prefersDark) {
        document.documentElement.classList.add('dark')
      } else {
        document.documentElement.classList.remove('dark')
      }
    }
  }

  // 切换左侧面板显示状态
  const toggleLeftPanel = () => {
    leftPanelVisible.value = !leftPanelVisible.value
  }

  // 设置左侧面板显示状态
  const setLeftPanelVisible = (visible: boolean) => {
    leftPanelVisible.value = visible
  }

  // 切换右侧面板显示状态
  const toggleRightPanel = () => {
    rightPanelVisible.value = !rightPanelVisible.value
  }

  // 设置右侧面板显示状态
  const setRightPanelVisible = (visible: boolean) => {
    rightPanelVisible.value = visible
  }

  // 初始化设置
  const initializeSettings = async () => {
    await loadSettings()
    applyTheme(settings.theme)

    // 监听系统主题变化
    if (settings.theme === 'auto') {
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
      mediaQuery.addEventListener('change', () => {
        if (settings.theme === 'auto') {
          applyTheme('auto')
        }
      })
    }
  }

  return {
    // 状态
    settings,
    isLoading,
    leftPanelVisible,
    rightPanelVisible,

    // 方法
    loadSettings,
    saveSettings,
    resetSettings,
    getSetting,
    setSetting,
    exportSettings,
    importSettings,
    applyTheme,
    initializeSettings,
    toggleLeftPanel,
    setLeftPanelVisible,
    toggleRightPanel,
    setRightPanelVisible,
  }
})
