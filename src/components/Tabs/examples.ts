/**
 * 标签页管理器使用示例
 * 
 * 这个文件展示了如何使用 useTabManagerStore 来管理多标签页功能
 */

import { useTabManagerStore } from '@/stores/tabManager'

// 在 Vue 组件中使用标签页管理器的示例
export function useTabManagerExample() {
  const tabManager = useTabManagerStore()

  // 示例1: 创建新标签页
  const createNewTabExample = async () => {
    try {
      // 创建一个新的插件标签页
      const tab = await tabManager.createNewTab('example-plugin', '我的新标签页')
      console.log('新标签页创建成功:', tab)
      
      // 标签页包含:
      // - id: 标签页唯一ID
      // - title: '我的新标签页'
      // - pluginId: 'example-plugin'
      // - instanceId: 自动生成的插件实例ID
      // - sessionId: 自动创建的会话ID
      // - isActive: true (新创建的标签页会自动激活)
      // - isPinned: false
      // - createdAt: 创建时间
      // - updatedAt: 更新时间
      
    } catch (error) {
      console.error('创建标签页失败:', error)
    }
  }

  // 示例2: 切换标签页
  const switchTabExample = async () => {
    try {
      const tabs = tabManager.tabs
      if (tabs.length > 1) {
        // 切换到第二个标签页
        await tabManager.switchToTab(tabs[1].id)
        console.log('已切换到标签页:', tabs[1].title)
      }
    } catch (error) {
      console.error('切换标签页失败:', error)
    }
  }

  // 示例3: 关闭标签页
  const closeTabExample = async () => {
    try {
      const activeTab = tabManager.activeTab
      if (activeTab && !activeTab.isPinned) {
        await tabManager.closeTab(activeTab.id)
        console.log('标签页已关闭:', activeTab.title)
      }
    } catch (error) {
      console.error('关闭标签页失败:', error)
    }
  }

  // 示例4: 重命名标签页
  const renameTabExample = () => {
    const activeTab = tabManager.activeTab
    if (activeTab) {
      const success = tabManager.renameTab(activeTab.id, '新的标签页名称')
      if (success) {
        console.log('标签页已重命名')
      }
    }
  }

  // 示例5: 固定/取消固定标签页
  const togglePinExample = () => {
    const activeTab = tabManager.activeTab
    if (activeTab) {
      const success = tabManager.toggleTabPin(activeTab.id)
      if (success) {
        console.log(`标签页已${activeTab.isPinned ? '取消固定' : '固定'}`)
      }
    }
  }

  // 示例6: 获取插件的会话统计
  const getPluginStatsExample = () => {
    const activeTab = tabManager.activeTab
    if (activeTab) {
      const stats = tabManager.getPluginSessionStats(activeTab.pluginId)
      console.log('插件会话统计:', {
        插件ID: stats.pluginId,
        总会话数: stats.totalSessions,
        总消息数: stats.totalMessages,
        最后活动: stats.lastActivity,
        活跃会话: stats.activeSessions
      })
    }
  }

  // 示例7: 批量操作
  const batchOperationsExample = async () => {
    try {
      // 关闭所有未固定的标签页
      const unpinnedTabs = tabManager.activeTabs
      for (const tab of unpinnedTabs) {
        await tabManager.closeTab(tab.id)
      }
      console.log('所有未固定标签页已关闭')
      
      // 或者使用便捷方法
      // await tabManager.closeAllTabs()
      
    } catch (error) {
      console.error('批量操作失败:', error)
    }
  }

  // 示例8: 监听标签页变化
  const watchTabChangesExample = () => {
    // 在 Vue 组件中使用 watch
    /*
    import { watch } from 'vue'
    
    watch(() => tabManager.activeTab, (newTab, oldTab) => {
      if (newTab) {
        console.log('切换到标签页:', newTab.title)
        // 可以在这里执行标签页切换后的逻辑
        // 比如更新页面标题、发送分析事件等
      }
    })
    
    watch(() => tabManager.tabs.length, (newCount, oldCount) => {
      console.log(`标签页数量变化: ${oldCount} -> ${newCount}`)
    })
    */
  }

  // 示例9: 键盘快捷键集成
  const keyboardShortcutsExample = () => {
    // 这些快捷键已经在 TabManager.vue 中实现
    console.log('支持的键盘快捷键:')
    console.log('Ctrl/Cmd + T: 新建标签页')
    console.log('Ctrl/Cmd + W: 关闭当前标签页')
    console.log('Ctrl/Cmd + 1-9: 切换到对应标签页')
    console.log('Ctrl/Cmd + Tab: 切换到下一个标签页')
    console.log('Ctrl/Cmd + Shift + Tab: 切换到上一个标签页')
    console.log('Ctrl/Cmd + Shift + T: 恢复最近关闭的标签页')
  }

  // 示例10: 标签页持久化
  const persistenceExample = () => {
    // 标签页状态会自动保存到 localStorage
    // 应用重启后会自动恢复标签页状态
    
    // 手动保存
    tabManager.saveToStorage()
    
    // 手动加载
    tabManager.loadFromStorage()
    
    console.log('标签页状态已保存到本地存储')
  }

  return {
    createNewTabExample,
    switchTabExample,
    closeTabExample,
    renameTabExample,
    togglePinExample,
    getPluginStatsExample,
    batchOperationsExample,
    watchTabChangesExample,
    keyboardShortcutsExample,
    persistenceExample
  }
}

// 使用示例
export function demonstrateTabManager() {
  const examples = useTabManagerExample()
  
  console.log('=== 标签页管理器功能演示 ===')
  
  // 依次执行各种示例
  setTimeout(() => examples.createNewTabExample(), 1000)
  setTimeout(() => examples.switchTabExample(), 2000)
  setTimeout(() => examples.renameTabExample(), 3000)
  setTimeout(() => examples.togglePinExample(), 4000)
  setTimeout(() => examples.getPluginStatsExample(), 5000)
  
  console.log('演示已开始，请查看控制台输出')
}

// 标签页最佳实践
export const tabManagerBestPractices = {
  // 1. 标签页数量控制
  maxTabs: 20, // 建议最大标签页数量
  
  // 2. 标签页命名规范
  namingConvention: {
    // 使用有意义的名称
    good: ['ChatGPT 对话', '代码审查', '文档编写'],
    bad: ['新标签页 1', '未命名', 'Tab']
  },
  
  // 3. 固定标签页使用场景
  pinningUseCases: [
    '常用的插件实例',
    '重要的长期对话',
    '参考文档或配置页面'
  ],
  
  // 4. 性能优化建议
  performanceTips: [
    '及时关闭不需要的标签页',
    '使用固定标签页功能减少重复创建',
    '定期清理历史会话数据',
    '避免在单个会话中积累过多消息'
  ],
  
  // 5. 用户体验建议
  uxTips: [
    '为标签页设置有意义的标题',
    '使用右键菜单进行快速操作',
    '利用键盘快捷键提高效率',
    '合理使用标签页分组功能（未来功能）'
  ]
}
