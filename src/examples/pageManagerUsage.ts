/**
 * 页面管理器使用示例
 * 
 * 这个文件展示了如何使用 usePageManagerStore 来管理页面状态
 */

import { usePageManagerStore } from '@/stores/pageManager'

// 在 Vue 组件中使用页面管理器的示例
export function usePageManagerExample() {
  const pageManager = usePageManagerStore()

  // 示例1: 创建新页面
  const createNewPageExample = async () => {
    try {
      // 创建一个新的插件页面
      const pageState = await pageManager.createNewPage('example-plugin', '我的新页面')
      console.log('新页面创建成功:', pageState)
      
      // 页面状态包含:
      // - pluginId: 'example-plugin'
      // - instanceId: 自动生成的UUID
      // - sessionId: 自动创建的会话ID
      // - title: '我的新页面'
      // - createdAt: 创建时间
      // - updatedAt: 更新时间
      
    } catch (error) {
      console.error('创建页面失败:', error)
    }
  }

  // 示例2: 切换到指定页面
  const switchToPageExample = async () => {
    try {
      // 获取页面历史记录
      const history = pageManager.getPageHistory()
      
      if (history.length > 0) {
        // 切换到最近的页面
        const recentPage = history[0].pageState
        await pageManager.switchToPage(recentPage)
        console.log('页面切换成功:', recentPage)
      }
    } catch (error) {
      console.error('页面切换失败:', error)
    }
  }

  // 示例3: 切换会话（在当前插件实例下）
  const switchToSessionExample = async () => {
    try {
      // 假设我们有一个会话ID
      const sessionId = 'session_123456'
      await pageManager.switchToSession(sessionId)
      console.log('会话切换成功')
    } catch (error) {
      console.error('会话切换失败:', error)
    }
  }

  // 示例4: 获取当前页面信息
  const getCurrentPageInfo = () => {
    const currentPage = pageManager.currentPage
    const currentPluginId = pageManager.currentPluginId
    const currentInstanceId = pageManager.currentInstanceId
    const currentSessionId = pageManager.currentSessionId
    const isValid = pageManager.isCurrentPageValid

    console.log('当前页面信息:', {
      currentPage,
      currentPluginId,
      currentInstanceId,
      currentSessionId,
      isValid
    })
  }

  // 示例5: 获取页面历史记录
  const getPageHistoryExample = () => {
    // 获取所有页面历史
    const allHistory = pageManager.getPageHistory()
    console.log('所有页面历史:', allHistory)

    // 获取特定插件的页面历史
    const pluginHistory = pageManager.getPluginPageHistory('example-plugin')
    console.log('Example插件的页面历史:', pluginHistory)
  }

  // 示例6: 管理历史记录
  const manageHistoryExample = () => {
    // 删除特定历史记录
    const history = pageManager.getPageHistory()
    if (history.length > 0) {
      const removed = pageManager.removeFromHistory(history[0].id)
      console.log('删除历史记录:', removed)
    }

    // 清空所有历史记录
    // pageManager.clearHistory()
    // console.log('历史记录已清空')
  }

  return {
    createNewPageExample,
    switchToPageExample,
    switchToSessionExample,
    getCurrentPageInfo,
    getPageHistoryExample,
    manageHistoryExample,
    
    // 直接暴露页面管理器实例，供组件使用
    pageManager
  }
}

// 在 Vue 组件的 setup 函数中使用示例:
/*
<script setup lang="ts">
import { usePageManagerExample } from '@/examples/pageManagerUsage'

const {
  createNewPageExample,
  switchToPageExample,
  getCurrentPageInfo,
  pageManager
} = usePageManagerExample()

// 监听当前页面变化
watch(() => pageManager.currentPage, (newPage) => {
  console.log('当前页面已更改:', newPage)
})

// 在模板中使用
const handleCreateNewPage = () => {
  createNewPageExample()
}

const handleSwitchPage = () => {
  switchToPageExample()
}
</script>

<template>
  <div>
    <button @click="handleCreateNewPage">创建新页面</button>
    <button @click="handleSwitchPage">切换页面</button>
    <button @click="getCurrentPageInfo">获取当前页面信息</button>
    
    <!-- 显示当前页面信息 -->
    <div v-if="pageManager.currentPage">
      <h3>当前页面</h3>
      <p>插件ID: {{ pageManager.currentPluginId }}</p>
      <p>实例ID: {{ pageManager.currentInstanceId }}</p>
      <p>会话ID: {{ pageManager.currentSessionId }}</p>
      <p>标题: {{ pageManager.currentPage.title }}</p>
    </div>
    
    <!-- 显示页面历史 -->
    <div>
      <h3>页面历史</h3>
      <ul>
        <li v-for="item in pageManager.getPageHistory()" :key="item.id">
          {{ item.pageState.title }} - {{ item.timestamp.toLocaleString() }}
          <button @click="pageManager.switchToPage(item.pageState)">切换</button>
        </li>
      </ul>
    </div>
  </div>
</template>
*/
